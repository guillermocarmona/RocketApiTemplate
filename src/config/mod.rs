use dotenvy::dotenv;
use dotenvy::from_filename;
use std::env;

pub fn init() {
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "production".to_string());

    match app_env.as_str() {
        "development" => {
            from_filename(".env.development").ok();
            println!("Using development configuration (.env.development)");
        }
        _ => {
            dotenv().ok(); // Carga el archivo predeterminado (.env)
            println!("Using production configuration (.env)");
        }
    }
}


pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL is not defined")
}