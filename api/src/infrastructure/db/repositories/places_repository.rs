use chrono::Utc;
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel::prelude::*;

use crate::errors::InfrastructureError;
use crate::infrastructure::db::models::{CreatePlaceRow, PlaceRow, UpdatePlaceRow};
use crate::infrastructure::db::schema::places;
use crate::models::ids::WorkspaceId;
use crate::models::place::{CreatePlace, Place, UpdatePlace};

#[derive(Clone)]
pub struct PlacesRepository {
    pub pool: Pool<Manager<PgConnection>>,
}

impl PlacesRepository {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn list_places(
        &self,
        workspace_id: WorkspaceId,
        search: &str,
    ) -> Result<Vec<Place>, InfrastructureError> {
        let search = search.to_string();

        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let place_list: Vec<PlaceRow> = connection
            .interact(move |conn| {
                let search_like = format!("%{}%", search);

                let mut places_query = places::table
                    .filter(places::deleted_at.is_null())
                    .filter(places::workspace_id.eq(workspace_id.value()))
                    .into_boxed();

                let filter_expression = places::name.ilike(&search_like);

                if let Ok(search_number) = search.parse::<i32>() {
                    places_query =
                        places_query.filter(filter_expression.or(places::id.eq(search_number)));
                } else {
                    places_query = places_query.filter(filter_expression);
                }

                places_query.load::<PlaceRow>(conn)
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        let prods = place_list
            .into_iter()
            .map(|place| place.into())
            .collect::<Vec<Place>>();

        Ok(prods)
    }

    pub async fn get_place_by_id(
        &self,
        workspace_id: WorkspaceId,
        place_id: i32,
    ) -> Result<Option<Place>, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let place = connection
            .interact(move |conn| {
                places::table
                    .filter(places::deleted_at.is_null())
                    .filter(places::workspace_id.eq(workspace_id.value()))
                    .find(place_id)
                    .first::<PlaceRow>(conn)
                    .optional()
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(place.map(|p| p.into()))
    }

    pub async fn create_place(&self, new_place: CreatePlace) -> Result<Place, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let create_place_row = CreatePlaceRow {
            workspace_id: new_place.workspace_id.value(),
            name: new_place.name,
        };

        let created_place = connection
            .interact(move |conn| {
                diesel::insert_into(places::table)
                    .values(create_place_row)
                    .returning(PlaceRow::as_returning())
                    .get_result::<PlaceRow>(conn)
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(created_place.into())
    }

    pub async fn update_place(
        &self,
        workspace_id: WorkspaceId,
        place_id: i32,
        place: UpdatePlace,
    ) -> Result<Option<Place>, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let update_place_row = UpdatePlaceRow { name: place.name };

        let now = Utc::now().naive_utc();

        let updated_place = connection
            .interact(move |conn| {
                diesel::update(
                    places::table
                        .filter(places::deleted_at.is_null())
                        .filter(places::workspace_id.eq(workspace_id.value()))
                        .find(place_id),
                )
                .set((&update_place_row, places::updated_at.eq(now)))
                .returning(PlaceRow::as_returning())
                .get_result(conn)
                .optional()
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(updated_place.map(|p| p.into()))
    }

    pub async fn delete_place(
        &self,
        workspace_id: WorkspaceId,
        place_id: i32,
    ) -> Result<Option<Place>, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let now = Utc::now().naive_utc();

        let deleted_place = connection
            .interact(move |conn| {
                diesel::update(
                    places::table
                        .filter(places::workspace_id.eq(workspace_id.value()))
                        .find(place_id),
                )
                .set(places::deleted_at.eq(Some(now)))
                .returning(PlaceRow::as_returning())
                .get_result(conn)
                .optional()
            })
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(deleted_place.map(|p| p.into()))
    }
}
