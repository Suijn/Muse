use uuid::Uuid;

use muse::users::models;

#[test]
fn user_create() {
    let user_id = Uuid::new_v4();
    let user = models::User {
        id: user_id,
        name: String::from("dummy_user"),
        email: String::from("dummy@gmail.com"),
        password: String::from("top_secret"),
        status: models::UserStatus::New,
    };
    assert_eq!(user_id, user.id);
    assert_eq!("dummy_user", user.name);
    assert_eq!("dummy@gmail.com", user.email);
    assert_eq!("top_secret", user.password);
    assert_eq!(models::UserStatus::New, user.status);
}
