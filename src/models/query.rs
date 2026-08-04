use serde::Deserialize;



#[derive(Deserialize)]
pub struct LimitOffsetQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>
}