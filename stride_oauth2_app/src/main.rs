// use rand::distributions::{Alphanumeric, DistString};

use actix_web::{get, post, web::{self, Json}, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

pub mod model;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn create_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    println!("Hello, world!");

    let dbConnection = create_connection();

    HttpServer::new(|| {
        App::new()
            .service(create_client)
            // .route("/devices/get", web::get().to(get_devices))
            // .route("/devices/create", web::post().to(create_device))
    })
        .bind(("127.0.0.1", 8080))?  // This app will run on http://127.0.0.1:8080/
        .run()
        .await

    /* let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    println!("Random String is: {}", string); */
}


//======================================= API's ===================================

#[derive(Serialize, Deserialize)]
pub struct CreateClientPayload{
    pub client_origin_url: String,
    pub redirect_url: String,
    pub scope: Vec<String>,
    pub org_name: String
}

#[post("/RegisterClient")]
async fn create_client(_req_body: Json<CreateClientPayload>) -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

//==================================================================================