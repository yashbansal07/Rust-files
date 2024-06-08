use std::fmt;

use super::Group;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub unique_user_code: String,
    pub groups: Vec<Group>,
}

impl<'c> FromRow<'c, MySqlRow<'c>> for User {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            unique_user_code: row.get(3),
            groups: Vec::with_capacity(0),
        })
    }
}

// impl fmt::Display for User {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Id: {}", self.id)
//     }
// }

impl fmt::Display for User {
    // #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // fmt::Display::fmt(&*self, f)
        write!(f, "User display function!")
    }
}