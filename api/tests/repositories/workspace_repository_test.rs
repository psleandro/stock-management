use talk_to_me_api::{
    db::workspace_repository::WorkspaceRepository, models::workspace::CreateWorkspace,
};

use crate::common::{
    db::{clean_db, create_test_pool, lock_test_db},
    helpers::create_user,
};

#[tokio::test]
async fn should_create_workspace() {
    let _lock = lock_test_db().await;
    let pool = create_test_pool().await;
    clean_db(pool).await;

    let user = create_user(pool, None).await;

    let ws_repository = WorkspaceRepository::new(pool.clone());

    let result = ws_repository
        .create_workspace(CreateWorkspace {
            name: user.name.clone(),
            owner_id: user.id,
        })
        .await
        .unwrap();

    assert_eq!(result.name, user.name);
    assert_eq!(result.owner_id.value(), user.id.value());
}
