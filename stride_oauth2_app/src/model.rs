use diesel::{Queryable, Insertable};

use super::schema::client_info;

#[derive(Queryable)]
pub struct ClientInfo {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub age: i32,
}

#[derive(Insertable)]
#[table_name = "client_info"]
pub struct NewClient<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub age: &'a i32,
}