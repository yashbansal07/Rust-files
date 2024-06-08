use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::client_info)]
pub struct ClientInfo {
    #[serde(default)]
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub age: i32,}