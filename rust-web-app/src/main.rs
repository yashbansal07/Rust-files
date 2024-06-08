use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/devices/get", web::get().to(get_devices))
            .route("/devices/create", web::post().to(create_device))
    })
        .bind(("127.0.0.1", 8080))?  // This app will run on http://127.0.0.1:8080/
        .run()
        .await
}

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

//Route of this is /hey
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[derive(Serialize, Deserialize)]
struct Device {
    id: i32,
    mac: String,
    firmware: String,
}

fn fibonacci(n: u32) -> u32 {
    match n {
        1 | 2 => 1,
        3 => 2,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

async fn get_devices() -> HttpResponse {
    let mut devices = Vec::new();

    devices.push(Device {id: 1, mac: String::from("5F-33-CC-1F-43-82"), firmware: String::from("2.1.6")});
    devices.push(Device {id: 2, mac: String::from("EF-2B-C4-F5-D6-34"), firmware: String::from("2.1.6")});

    HttpResponse::Ok().json(devices)
}

// #[post("/devices/create")]
async fn create_device() -> impl Responder {
    let number = 3;
    // let number = req_body;

    let fib = fibonacci(number);
    let location = format!("/devices/{}", fib);

    HttpResponse::Created()
        .append_header(("location", location))
        // .body("In Response Header, Locn key is appneded!")
        .finish()
}