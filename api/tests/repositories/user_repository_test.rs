#[cfg(test)]
mod tests {
    use talk_to_me_api::{
        db::{user_repository::UserRepository, workspace_repository::WorkspaceRepository},
        models::{user::CreateUser, workspace::CreateWorkspace},
    };

    use crate::common::{
        db::{clean_db, create_test_pool, lock_test_db},
        helpers::create_user,
    };

    #[tokio::test]
    async fn should_create_user_from_pool() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_name = "John Doe".to_string();
        let user_email = "johndoe@email.com".to_string();

        let user_payload = CreateUser {
            name: user_name.clone(),
            email: user_email.clone(),
            password: "johndoepassword".to_string(),
        };

        let result = create_user(pool, Some(user_payload)).await;

        assert_eq!(result.name, Some(user_name));
        assert_eq!(result.email, user_email);
    }

    #[tokio::test]
    async fn should_return_user_when_he_has_workspace() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_repository = UserRepository::new(pool.clone());
        let ws_repository = WorkspaceRepository::new(pool.clone());

        let created_user = create_user(pool, None).await;
        let user_workspace = ws_repository
            .create_workspace(CreateWorkspace {
                name: created_user.name.clone(),
                owner_id: created_user.id,
            })
            .await
            .unwrap();

        let result = user_repository
            .get_user_by_email(created_user.email.clone())
            .await
            .unwrap();

        assert!(result.is_some());

        let user_found = result.unwrap();

        assert_eq!(user_found.workspace_id, user_workspace.id);
        assert_eq!(user_found.id, created_user.id);
        assert_eq!(user_found.name, created_user.name);
        assert_eq!(user_found.email, created_user.email);
        assert_eq!(user_found.created_at, created_user.created_at);
    }

    #[tokio::test]
    async fn should_return_none_when_user_not_exists() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_repository = UserRepository::new(pool.clone());

        let result = user_repository
            .get_user_by_email("non-existing-user@email.com".to_string())
            .await
            .unwrap();

        assert!(result.is_none())
    }

    #[tokio::test]
    async fn should_return_none_when_user_not_has_workspace() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let created_user = create_user(pool, None).await;

        let user_repository = UserRepository::new(pool.clone());

        let result = user_repository
            .get_user_by_email(created_user.email.clone())
            .await
            .unwrap();

        assert!(result.is_none());
    }
}
