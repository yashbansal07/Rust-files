// use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};

#[derive(Serialize, Deserialize, Clone)]
pub struct Clients {
    pub id: i32,
    pub org_name: String,
    pub email: String,
    pub client_origin_url: String,
    pub redirect_url: String,
    pub client_id: String,
    pub client_secret: String,
}

impl<'c> FromRow<'c, MySqlRow<'c>> for Clients {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Clients {
            id: row.get(0),
            org_name: row.get(1),
            email: row.get(2),
            client_origin_url: row.get(3),
            redirect_url: row.get(4),
            client_id: row.get(5),
            client_secret: row.get(6),
        })
    }
}

// impl fmt::Display for User {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Id: {}", self.id)
//     }
// }

