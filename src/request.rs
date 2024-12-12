mod backend_async {
    pub mod request;
}

pub struct LoginRequest {
    pub username: String,
    pub password: String,
}