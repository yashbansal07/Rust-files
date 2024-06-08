use actix_web::{web, get, post, delete, put, HttpResponse};
use crate::{models::client_info::ClientInfo, repository::database::Database};


/*#[post("/todos")]
pub async fn create_client(db: web::Data<Database>, new_todo: web::Json<Todo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}*/

#[get("/getClient")]
pub async fn get_client(db: web::Data<Database>) -> HttpResponse {
    let clients = db.get_clients();
    HttpResponse::Ok().json(clients)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_client)
    );
}
