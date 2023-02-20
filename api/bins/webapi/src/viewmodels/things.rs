use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DelayedThingsRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DelayedThingsResponse {
    pub id: String,
}
