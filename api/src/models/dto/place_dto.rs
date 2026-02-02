use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct ListPlacesParams {
    pub search: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct CreatePlaceDto {
    #[validate(length(min = 2, message = "must be at least 2 characters long"))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdatePlaceDto {
    #[validate(length(min = 2, message = "must be at least 2 characters long"))]
    pub name: Option<String>,
}
