use actix_web::{
    HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use tracing::{info, instrument};

use crate::{config::services::Services, errors::AppError};

#[instrument(name = "auth_middleware", skip(req, next))]
pub async fn auth(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let services = req
        .app_data::<web::Data<Services>>()
        .ok_or_else(|| actix_web::Error::from(AppError::Unauthorized))?;
    
    let refresh_token = req
        .cookie("refresh_t")
        .map(|c| c.value().to_string())
        .ok_or_else(|| actix_web::Error::from(AppError::Unauthorized))?;

    let claims = services
        .token
        .verify_refresh_token(&refresh_token)
        .map_err(|e| {
            tracing::warn!(error = ?e, "refresh token verification failed");
            actix_web::Error::from(AppError::Unauthorized)
        })?;
    req.extensions_mut().insert(claims.sub);

    let res = next
        .call(req)
        .await?;
    Ok(res)
}
