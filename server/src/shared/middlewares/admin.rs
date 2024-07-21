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

use super::super::entities::profesor_db::ProfessorDB;

// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Admin;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Admin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
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
                let err = error::ErrorUnauthorized("Authorization header not found").into();
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

        //if auth != "token" {
        //    let err = error::ErrorUnauthorized("Invalid token").into();
        //    return Box::pin(async { Err(err) });
        //}
        if let Err(e) = check_token(token) {
            let err = error::ErrorUnauthorized(e.1).into();
            return Box::pin(async { Err(err) });
        }

        let fut = self.service.call(req);
        Box::pin(async move { fut.await })
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    user: ProfessorDB,
    exp: usize,
}

fn check_token(token: &str) -> Result<(), (u16, String)> {
    dotenv().ok();
    let secret = match env::var("SEED_JWT") {
        Ok(v) => v,
        Err(e) => return Err((500, e.to_string())),
    };

    let mut validate = Validation::default();
    validate.validate_exp = false;

    let professor =
        match decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validate) {
            Ok(t) => t.claims.user,
            Err(e) => return Err((500, format!("Error to decode: {}", e.to_string()))),
        };

    if !professor.is_admin() {
        return Err((401, "Not admin".to_string()));
    }

    Ok(())
}
