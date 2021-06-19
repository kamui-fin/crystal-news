use std::pin::Pin;
use std::task::{Context, Poll};

use crate::util::jwt::validate;
use crate::{config::Context as ApiContext, util::get_bearer};
use actix_web::dev::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, web, Error, HttpResponse};
use futures::future::{ok, Ready};
use futures::Future;

pub struct Authorization;

impl<S, B> Transform<S> for Authorization
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthorizationMiddleware { service })
    }
}

pub struct AuthorizationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthorizationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers();
        let mut validity: bool = false;
        let bearer = get_bearer(headers);
        let secret = req.app_data::<web::Data<ApiContext>>().unwrap(); // data context will be on all routes
        if let Some(bearer) = bearer {
            validity = validate(bearer, &secret.config.jwt_secret);
        }

        if validity {
            let fut = self.service.call(req);
            // allow user to access the route
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        } else {
            Box::pin(async move {
                Ok(req.into_response(HttpResponse::Unauthorized().body("").into_body()))
            })
        }
    }
}
