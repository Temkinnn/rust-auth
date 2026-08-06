use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use tracing::{info, instrument};

use crate::{config::services::Services, errors::AppError};

#[instrument(skip(req, next))]
pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let services = req
        .app_data::<web::Data<Services>>()
        .ok_or_else(|| actix_web::Error::from(AppError::Internal))?;

    let token = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| actix_web::Error::from(AppError::Unauthorized))?
        .to_str()
        .map_err(|_| actix_web::Error::from(AppError::Internal))?
        .strip_prefix("Bearer ")
        .ok_or_else(|| actix_web::Error::from(AppError::Unauthorized))?
        .to_string();

    let claims = services.token.verify_access_token(&token)?;

    let user_id = claims.sub;

    req.extensions_mut().insert(user_id);

    info!("User id: {}", user_id);

    let res = next.call(req).await?;
    Ok(res)
}
