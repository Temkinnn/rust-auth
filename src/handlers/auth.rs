use actix_web::{
    HttpRequest, HttpResponse, Responder,
    cookie::{Cookie, SameSite},
    delete, post, put,
    web::{self, ServiceConfig, scope},
};

use crate::{
    config::services::Services,
    errors::AppError,
    models::auth::{LoginCredentials, RegistrationCredentials},
    types::AppResult,
};

#[post("/login")]
async fn login(
    services: web::Data<Services>,
    data: web::Json<LoginCredentials>,
) -> AppResult<impl Responder> {
    let tokens = services.auth.login(data.into_inner()).await?;

    let cookie = Cookie::build("refresh_t", &tokens.refresh_token)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(tokens))
}

#[post("/register")]
async fn register(
    services: web::Data<Services>,
    data: web::Json<RegistrationCredentials>,
) -> AppResult<impl Responder> {
    let tokens = services.auth.register(data.into_inner()).await?;

    let cookie = Cookie::build("refresh_t", &tokens.refresh_token)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    Ok(HttpResponse::Created().cookie(cookie).json(tokens))
}

#[put("/refresh")]
async fn refresh(req: HttpRequest, services: web::Data<Services>) -> AppResult<impl Responder> {
    let refresh_token = req
        .cookie("refresh_t")
        .map(|c| c.value().to_string())
        .ok_or(AppError::Unauthorized)?;

    let tokens = services.auth.refresh(refresh_token).await?;

    let cookie = Cookie::build("refresh_t", &tokens.refresh_token)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(tokens))
}

#[delete("/logout")]
async fn logout(req: HttpRequest, services: web::Data<Services>) -> AppResult<impl Responder> {
    let refresh_token = req
        .cookie("refresh_t")
        .map(|c| c.value().to_string())
        .ok_or(AppError::Unauthorized)?;

    services.auth.logout(refresh_token).await?;

    Ok(HttpResponse::Ok())
}

pub fn auth_router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(register)
            .service(login)
            .service(refresh)
            .service(logout),
    );
}
