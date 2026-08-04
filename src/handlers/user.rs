use actix_web::{
    HttpRequest, HttpResponse, Responder, get,
    middleware::from_fn,
    post,
    web::{self, ServiceConfig, scope},
};
use uuid::Uuid;

use crate::{
    config::services::Services,
    errors::AppError,
    middlewares::{admin::admin_middleware, auth::auth_middleware},
    models::{query::LimitOffsetQuery, user::CreateUserDto},
    types::AppResult,
};

#[get("/me")]
async fn me(req: HttpRequest, services: web::Data<Services>) -> AppResult<impl Responder> {
    use actix_web::HttpMessage;

    let user_id = *req
        .extensions()
        .get::<Uuid>()
        .ok_or(AppError::Unauthorized)?;

    let user = services.user.get_user_by_id(user_id).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/")]
async fn get_users(
    services: web::Data<Services>,
    query: web::Query<LimitOffsetQuery>,
) -> AppResult<impl Responder> {
    let users = services.user.get_users(query.limit, query.offset).await?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("/")]
async fn create_user(
    services: web::Data<Services>,
    body: web::Json<CreateUserDto>,
) -> AppResult<impl Responder> {
    let users = services.user.create_user(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(users))
}

pub fn users_router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users") // auth_middleware
            .service(me)
            .wrap(from_fn(auth_middleware))
            .service(get_users)
            .wrap(from_fn(auth_middleware)),
    )
    .service(
        scope("/users") // admin_middleware
            .service(create_user)
            .wrap(from_fn(admin_middleware)),
    );
}
