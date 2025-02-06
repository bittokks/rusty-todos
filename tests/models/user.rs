use todos::models::users::{RegisterUser, User};

#[tokio::test]
async fn test_register_user_success() {
    let dto = RegisterUser::new("test_username", "test@example.com", "Password", "Password");

    // let actual_user = User::register(db, &dto).await;
}
