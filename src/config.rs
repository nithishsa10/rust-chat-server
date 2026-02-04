use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url:            String,
    pub redis_url:               String,
    pub jwt_secret:              String,
    pub jwt_expiry_secs:         i64,
    pub jwt_refresh_expiry_secs: i64,
    pub host:                    String,
    pub port:                    u16,
}


impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            database_url:            env::var("DATABASE_URL")?,
            redis_url:               env::var("REDIS_URL")?,
            jwt_secret:              env::var("JWT_SECRET")?,
            jwt_expiry_secs:         env::var("JWT_EXPIRY_SECS")?.parse()?,
            jwt_refresh_expiry_secs: env::var("JWT_REFRESH_EXPIRY_SECS")?.parse()?,
            host:                    env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port:                    env::var("PORT").unwrap_or_else(|_| "8080".into()).parse()?,
        })
    }
}