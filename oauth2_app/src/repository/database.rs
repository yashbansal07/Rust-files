// use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::client_info::ClientInfo;
use crate::repository::schema::client_info::dsl::*;


pub type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_clients(&self) -> Vec<ClientInfo> {
        client_info
            .load::<ClientInfo>(&mut self.pool.get().unwrap())
            .expect("Error loading all Clients")
    }
}