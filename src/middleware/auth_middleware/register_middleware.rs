use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Payload, Transform},
    Error, HttpResponse, web, body::BoxBody,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use validator::Validate;
use serde_json::json;
use actix_web::web::Bytes;
use actix_http::h1;

use crate::types::incoming_requests::RegisterRequest;

pub struct ValidateRegister;

impl<S> Transform<S, ServiceRequest> for ValidateRegister
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = ValidateRegisterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ValidateRegisterMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct ValidateRegisterMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for ValidateRegisterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
    
        Box::pin(async move {
            // Extract the JSON body
            let payload = req.extract::<web::Json<RegisterRequest>>().await;

            match payload {
                Ok(body) => {
                    // Validate the extracted data
                    if let Err(e) = body.validate() {
                        let error_message = e.to_string();
                        let response = HttpResponse::BadRequest()
                            .json(json!({"message": error_message}))
                            .map_into_boxed_body(); // Convert to `BoxBody`
                        return Ok(ServiceResponse::new(req.into_parts().0, response));
                    }
                    // Convert the validated RegisterRequest back into JSON bytes
                    let json_bytes = serde_json::to_vec(&*body).unwrap();

                    // Create a new payload from the bytes
                    let (mut sender, new_payload) = h1::Payload::create(true);
                    sender.feed_data(Bytes::from(json_bytes));
                    sender.feed_eof();

                    req.set_payload(Payload::from(new_payload));

                    // Proceed to the next service if validation passes
                    service.call(req).await
                }
                Err(_) => {
                    let response = HttpResponse::BadRequest()
                        .json(json!({"message": "Invalid register request body"}))
                        .map_into_boxed_body(); // Convert to `BoxBody`
                    Ok(ServiceResponse::new(req.into_parts().0, response))
                }
            }
        })
    }
}
