#[derive(Debug)]
pub enum Password {
    ClearText(String),
    Hashed(String),
}

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: Password,
}
