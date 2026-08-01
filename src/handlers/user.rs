use actix_web::{
    HttpRequest, HttpResponse, Responder, get,
    middleware::from_fn,
    web::{self, ServiceConfig, scope},
};
use uuid::Uuid;

use crate::{
    config::services::Services, errors::AppError, midlewares::auth::auth, types::AppResult,
};

#[get("/me")]
async fn me(services: web::Data<Services>, req: HttpRequest) -> AppResult<impl Responder> {
    use actix_web::HttpMessage;

    let user_id = *req
        .extensions()
        .get::<Uuid>()
        .ok_or(AppError::Unauthorized)?;

    let user = services.user.get_user_by_id(user_id).await?;

    Ok(HttpResponse::Ok().json(user))
}

pub fn user_router(cfg: &mut ServiceConfig) {
    cfg.service(scope("/user").service(me).wrap(from_fn(auth)));
}
