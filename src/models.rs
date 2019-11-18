#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first: String,
    pub last: String,
    pub password: String,
}