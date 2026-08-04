use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{config::services::Services, errors::AppError, models::user::Role};

#[instrument(skip(req, next))]
pub async fn admin_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let services = req
        .app_data::<web::Data<Services>>()
        .ok_or_else(|| actix_web::Error::from(AppError::Internal))?;

    let user_id = *req
        .extensions()
        .get::<Uuid>()
        .ok_or(AppError::Unauthorized)?;

    info!("User id: {}", user_id);

    match services.user.get_user_role_by_id(user_id).await? {
        Role::User => Err(actix_web::Error::from(AppError::Forbidden)),
        Role::Admin => Ok(next.call(req).await?),
    }
}
