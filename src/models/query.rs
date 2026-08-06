use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct LimitOffsetQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
