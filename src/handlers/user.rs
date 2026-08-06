use actix_web::{
    HttpRequest, HttpResponse, Responder, delete, get, middleware::from_fn, patch, post, web,
};
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use uuid::Uuid;

use crate::{
    config::services::Services,
    errors::AppError,
    middlewares::{admin::admin_middleware, auth::auth_middleware},
    models::{
        query::LimitOffsetQuery,
        user::{CreateUserDto, UpdateUserDto, User},
    },
    types::{AppResult, Id},
};

#[utoipa::path(
    tag = "User",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, body = User),
        (status = 401),
        (status = 404),
    )
)]
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

#[utoipa::path(
    tag = "User",
    params(LimitOffsetQuery),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, body = [User]),
        (status = 401),
    )
)]
#[get("/")]
async fn get_users(
    services: web::Data<Services>,
    query: web::Query<LimitOffsetQuery>,
) -> AppResult<impl Responder> {
    let users = services.user.get_users(query.limit, query.offset).await?;
    Ok(HttpResponse::Ok().json(users))
}

#[utoipa::path(
    tag = "User",
    params(
        ("id" = Uuid, Path, description = "The Unique user Id"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, body = [User]),
        (status = 401),
    )
)]
#[get("/{id}")]
async fn get_user_by_id(
    services: web::Data<Services>,
    path: web::Path<Id>,
) -> AppResult<impl Responder> {
    let user = services.user.get_user_by_id(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    tag = "User",
    request_body = CreateUserDto,
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 201, body = [User]),
        (status = 401),
    )
)]
#[post("/")]
async fn create_user(
    services: web::Data<Services>,
    body: web::Json<CreateUserDto>,
) -> AppResult<impl Responder> {
    let users = services.user.create_user(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(users))
}

#[utoipa::path(
    tag = "User",
    request_body = UpdateUserDto,
    params(
        ("id" = Uuid, Path, description = "The Unique user Id"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, body = [User]),
        (status = 401),
    )
)]
#[patch("/{id}")]
async fn update_user(
    services: web::Data<Services>,
    path: web::Path<Id>,
    body: web::Json<UpdateUserDto>,
) -> AppResult<impl Responder> {
    let user = services
        .user
        .update_user(path.into_inner(), body.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    tag = "User",
    params(
        ("id" = Uuid, Path, description = "The Unique user Id"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, body = [User]),
        (status = 401),
    )
)]
#[delete("/{id}")]
async fn delete_user(
    services: web::Data<Services>,
    path: web::Path<Id>,
) -> AppResult<impl Responder> {
    let user = services.user.delete_user(path.into_inner()).await?;
    Ok(HttpResponse::Created().json(user))
}

pub fn users_router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .wrap(from_fn(auth_middleware))
            .service(me)
            .wrap(from_fn(auth_middleware))
            .service(get_users)
            .wrap(from_fn(auth_middleware))
            .service(get_user_by_id)
            .wrap(from_fn(auth_middleware))
            .service(create_user)
            .wrap(from_fn(admin_middleware))
            .wrap(from_fn(auth_middleware))
            .service(delete_user)
            .wrap(from_fn(admin_middleware))
            .wrap(from_fn(auth_middleware))
            .service(update_user)
            .wrap(from_fn(admin_middleware))
            .wrap(from_fn(auth_middleware))
    );
}
