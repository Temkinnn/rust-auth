use actix_web::{
    HttpResponse, Responder, post,
    web::{self, ServiceConfig, scope},
};

use crate::{
    config::services::Services,
    models::auth::{LoginCredentials, RegistrationCredentials},
    types::AppResult,
};

#[post("/login")]
async fn login(
    services: web::Data<Services>,
    data: web::Json<LoginCredentials>,
) -> AppResult<impl Responder> {
    let tokens = services.auth.login(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(tokens))
}

#[post("/register")]
async fn register(
    services: web::Data<Services>,
    data: web::Json<RegistrationCredentials>,
) -> AppResult<impl Responder> {
    let tokens = services.auth.register(data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(tokens))
}

pub fn auth_router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").service(register).service(login));
}
