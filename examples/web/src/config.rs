#![allow(non_snake_case)]

itconfig::config! {
    #![config(unwrap)]

    // RUST_BACKTRACE => "1",

    RUST_LOG => "error",

    database {
        URL,

        pool {
            MAX_SIZE: u32 => 15,
        }
    }

    server {
        PORT: u16 => 8000,
    },

    feature {
        static CORS: bool => false,
    }
}

/// Util for configure application via env variables.
///
/// * Reads .env file for configure application (only for `dev` feature)
/// * Enables log macros via `env_logger` (See: https://docs.rs/env_logger)
/// * Initializes env config (See: https://docs.rs/itconfig)
///
/// Note: When enabled `dev` feature, this function try to load .env file
/// for configure application. If .env file cannot read will panic.
pub fn load_env_config() {
    #[cfg(feature = "dev")]
    dotenv::dotenv().expect("Cannot load .env file");

    init();

    env_logger::init();

    #[cfg(feature = "dev")]
    debug!("Env variables from .env file loaded successfully");

    debug!("Env configuration loaded successfully");
}
