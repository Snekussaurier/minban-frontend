mod utils {
    pub mod response;
}

pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
}