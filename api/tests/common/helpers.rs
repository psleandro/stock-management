use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::{Fake, Faker};
use talk_to_me_api::models::{
    ids::WorkspaceId,
    place::{CreatePlace, Place},
    product::{BaseUnit, CreateProduct, Product},
    supplier::{CreateSupplier, Supplier},
    user::{CreateUser, User},
    workspace::{CreateWorkspace, Workspace},
};

#[cfg(test)]
pub async fn create_user(pool: &Pool<Manager<PgConnection>>, payload: Option<CreateUser>) -> User {
    use talk_to_me_api::db::user_repository::UserRepository;

    use crate::common::helpers::random_user_payload;

    let user_repository = UserRepository::new(pool.clone());

    let user_payload = payload.unwrap_or_else(random_user_payload);

    let result = user_repository.create_user(user_payload).await.unwrap();

    result
}

#[cfg(test)]
pub async fn create_workspace(
    pool: &Pool<Manager<PgConnection>>,
    owner_id: talk_to_me_api::models::ids::UserId,
    name: Option<String>,
) -> Workspace {
    use talk_to_me_api::db::workspace_repository::WorkspaceRepository;

    let ws_repository = WorkspaceRepository::new(pool.clone());

    ws_repository
        .create_workspace(CreateWorkspace { name, owner_id })
        .await
        .unwrap()
}

#[cfg(test)]
pub async fn create_product(
    pool: &Pool<Manager<PgConnection>>,
    workspace_id: WorkspaceId,
    name: &str,
) -> Product {
    use talk_to_me_api::db::products_repository::ProductsRepository;

    let repo = ProductsRepository::new(pool.clone());

    repo.create_product(CreateProduct {
        workspace_id,
        name: name.to_string(),
        base_unit: BaseUnit::Unit,
        brand: Some("ACME".to_string()),
        min_stock: 5,
        observation: Some("obs".to_string()),
    })
    .await
    .unwrap()
}

#[cfg(test)]
pub async fn create_supplier(
    pool: &Pool<Manager<PgConnection>>,
    workspace_id: WorkspaceId,
    name: &str,
) -> Supplier {
    use talk_to_me_api::db::suppliers_repository::SuppliersRepository;

    let repo = SuppliersRepository::new(pool.clone());

    repo.create_supplier(CreateSupplier {
        workspace_id,
        name: name.to_string(),
    })
    .await
    .unwrap()
}

#[cfg(test)]
pub async fn create_place(
    pool: &Pool<Manager<PgConnection>>,
    workspace_id: WorkspaceId,
    name: &str,
) -> Place {
    use talk_to_me_api::db::places_repository::PlacesRepository;

    let repo = PlacesRepository::new(pool.clone());

    repo.create_place(CreatePlace {
        workspace_id,
        name: name.to_string(),
    })
    .await
    .unwrap()
}

fn random_user_payload() -> CreateUser {
    CreateUser {
        name: Name(PT_BR).fake(),
        email: fake::faker::internet::raw::SafeEmail(EN).fake(),
        password: Faker.fake(),
    }
}
