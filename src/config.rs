use std::env;

pub fn jwt_access_secret() -> Vec<u8> {
    env::var("JWT_ACCESS_SECRET")
        .expect("JWT_ACCESS_SECRET must be set")
        .into_bytes()
}

pub fn jwt_refresh_secret() -> Vec<u8> {
    env::var("JWT_REFRESH_SECRET")
        .expect("JWT_REFRESH_SECRET must be set")
        .into_bytes()
}

pub const UNIQUE_VIOLATION: &str = "23505";
