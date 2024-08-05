use actix_web::HttpMessage;
use dotenv::dotenv;
use std::env;
use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error, Error,
};

use serde::{Deserialize, Serialize};

use futures_util::future::LocalBoxFuture;

use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::shared::entities::user::User;

// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct StudentAuth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for StudentAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = StudentMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(StudentMiddleware { service }))
    }
}

pub struct StudentMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for StudentMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth = match req.headers().get("Authorization") {
            Some(auth) => auth,
            None => {
                let err = error::ErrorBadRequest("Authorization header not found").into();
                return Box::pin(async { Err(err) });
            }
        };

        let token = match auth.to_str() {
            Ok(auth) => auth,
            Err(_) => {
                let err =
                    error::ErrorInternalServerError("Authorization header is not a string").into();
                return Box::pin(async { Err(err) });
            }
        };

        let id = match get_id(token) {
            Ok(id) => id,
            Err((_, msg)) => {
                let err = error::ErrorUnauthorized(msg).into();
                return Box::pin(async { Err(err) });
            }
        };

        req.extensions_mut().insert(id);

        let fut = self.service.call(req);
        Box::pin(async move { fut.await })
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    user: User,
    exp: usize,
}

fn get_id(token: &str) -> Result<String, (u16, String)> {
    dotenv().ok();
    let secret = match env::var("SEED_JWT") {
        Ok(v) => v,
        Err(e) => return Err((500, e.to_string())),
    };

    let mut validate = Validation::default();
    validate.validate_exp = false;

    let student =
        match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validate) {
            Ok(t) => t.claims.user,
            Err(e) => return Err((500, format!("Token Student Error: {}", e.to_string()))),
        };

    Ok(student.id)
}
