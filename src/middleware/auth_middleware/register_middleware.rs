use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures::future::{ok, Ready, LocalBoxFuture};
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct ValidateRegister;

impl<S, B> Transform<S, ServiceRequest> for ValidateRegister
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
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

impl<S, B> Service<ServiceRequest> for ValidateRegisterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Indicate that the service is always ready to accept a request
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Here you can add validation logic for the request
            // e.g., check for required fields in JSON body, etc.
            // If valid, proceed to the next middleware or handler
            service.call(req).await
        })
    }
}
