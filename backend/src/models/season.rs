use sqlx::Type;

#[derive(Debug, Type)]
pub struct Season {
    pub id: i32,
    pub name: String,
    pub label: String,
}
