use crate::errors::ApplicationError;
use crate::infrastructure::db::places_repository::PlacesRepository;
use crate::models::dto::place_dto::{CreatePlaceDto, ListPlacesParams, UpdatePlaceDto};
use crate::models::ids::WorkspaceId;
use crate::models::place::{CreatePlace, Place, UpdatePlace};

#[derive(Clone)]
pub struct PlacesService {
    pub places_repository: PlacesRepository,
}

impl PlacesService {
    pub fn new(places_repository: PlacesRepository) -> Self {
        Self { places_repository }
    }

    pub async fn list_places(
        &self,
        workspace_id: WorkspaceId,
        params: ListPlacesParams,
    ) -> Result<Vec<Place>, ApplicationError> {
        let search = params.search.unwrap_or_default();

        let places = self
            .places_repository
            .list_places(workspace_id, &search)
            .await?;

        Ok(places)
    }

    pub async fn get_place(
        &self,
        workspace_id: WorkspaceId,
        place_id: i32,
    ) -> Result<Place, ApplicationError> {
        let place = self
            .places_repository
            .get_place_by_id(workspace_id, place_id)
            .await?;

        match place {
            Some(p) => Ok(p),
            None => Err(ApplicationError::NotFound),
        }
    }

    pub async fn create_place(
        &self,
        workspace_id: WorkspaceId,
        payload: CreatePlaceDto,
    ) -> Result<Place, ApplicationError> {
        let create_place_data = CreatePlace {
            workspace_id: workspace_id,
            name: payload.name,
        };

        let created_place = self
            .places_repository
            .create_place(create_place_data)
            .await?;

        Ok(created_place)
    }

    pub async fn update_place(
        &self,
        workspace_id: WorkspaceId,
        place_id: i32,
        payload: UpdatePlaceDto,
    ) -> Result<Place, ApplicationError> {
        let update_place_data = UpdatePlace { name: payload.name };

        let updated_place = self
            .places_repository
            .update_place(workspace_id, place_id, update_place_data)
            .await?;

        match updated_place {
            Some(p) => Ok(p),
            None => Err(ApplicationError::NotFound),
        }
    }

    pub async fn delete_place(
        &self,
        workspace_id: WorkspaceId,
        place_id: i32,
    ) -> Result<(), ApplicationError> {
        let deleted_place = self
            .places_repository
            .delete_place(workspace_id, place_id)
            .await?;

        match deleted_place {
            Some(_) => Ok(()),
            None => Err(ApplicationError::NotFound),
        }
    }
}
