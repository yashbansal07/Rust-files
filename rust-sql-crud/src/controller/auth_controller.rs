use crate::model::Clients;

use super::log_request;
use super::AppState;
// use crate::model::User;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_unique_token);
    cfg.service(get_client);
}

#[get("/userUniqueToken/{email}")]
async fn get_user_unique_token(user_email: web::Path<String>, app_state: web::Data<AppState<'_>>,) -> impl Responder {
    log_request("GET: /userUniqueToken", &app_state.connections);
    println!("In get_user_unique_token | Input email: {}", user_email);

    let user_code = app_state.context.users.get_user_unique_code_by_email(&user_email).await;

    match user_code {
        Err(_) => HttpResponse::InternalServerError().finish(),
        Ok(user_code) => {
            // let j = serde_json::to_string(&user_code);
            // println!("In get_user_unique_token | Data From DbQuery: {:?}", j);
            println!("In get_user_unique_token | Data From DbQuery: {}", user_code.unique_user_code);

            // HttpResponse::Ok().json(user_code)
            HttpResponse::Ok().body(user_code.unique_user_code)
        }
    }
}

#[get("/getAllClients")]
pub async fn get_client(app_state: web::Data<AppState<'_>>) -> HttpResponse {
    log_request("GET: /getAllClients", &app_state.connections);

    let clients = app_state.context.users.get_all_clients().await;

    match clients {
        Err(_) => HttpResponse::InternalServerError().finish(),
        Ok(clients) => {
            // let json = serde_json::to_string(&clients)?;
            let json = json!(clients);

            HttpResponse::Ok().json(json)
        }
    }
}