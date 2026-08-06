use actix_web::{
    HttpRequest, HttpResponse, Responder,
    cookie::{Cookie, SameSite},
    delete, post, put, web,
};
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    config::services::Services,
    errors::AppError,
    models::{
        auth::{LoginCredentials, RegistrationCredentials},
        token::Tokens,
    },
    types::AppResult,
};

#[utoipa::path(
    tag = "Auth",
    request_body = LoginCredentials,
    responses(
        (status = 200, body = Tokens),
        (status = 400)
    )
)]
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


#[utoipa::path(
    tag = "Auth",
    request_body = RegistrationCredentials,
    responses(
        (status = 201, body = Tokens),
        (status = 400),
        (status = 404),
    )
)]
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

#[utoipa::path(
    tag = "Auth",
    responses(
        (status = 200, body = Tokens),
        (status = 401),
        (status = 404),
    )
)]

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

#[utoipa::path(
    tag = "Auth",
    responses(
        (status = 201),
        (status = 401),
        (status = 404),
    )
)]

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
