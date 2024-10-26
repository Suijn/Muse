use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub enum UserStatus {
    New,
    Active,
    Inactive,
    Suspected,
    Banned,
}

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub status: UserStatus,
}
