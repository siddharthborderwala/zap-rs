use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use zap_core::Zap;

struct ZapState {
    zap: Mutex<Zap>,
}

#[get("/get/{key}")]
async fn get(zap: web::Data<ZapState>, key: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(
        zap.zap
            .lock()
            .unwrap()
            .get(key.into_inner().as_str())
            .unwrap_or("nil".to_string()),
    )
}

#[post("/set/{key}")]
async fn set(zap: web::Data<ZapState>, key: web::Path<String>, req_body: String) -> impl Responder {
    let mut zap_data = zap.zap.lock().unwrap();
    (*zap_data).set(key.into_inner(), req_body);
    HttpResponse::Ok().body("ok")
}

#[post("/delete/{key}")]
async fn delete(zap: web::Data<ZapState>, key: web::Path<String>) -> impl Responder {
    let mut zap_data = zap.zap.lock().unwrap();
    (*zap_data).delete(key.into_inner());
    HttpResponse::Ok().body("ok")
}

#[get("/has/{key}")]
async fn has(zap: web::Data<ZapState>, key: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(zap.zap.lock().unwrap().has(key.into_inner()).to_string())
}

#[get("/list")]
async fn list(zap: web::Data<ZapState>) -> impl Responder {
    // list all key value pairs
    let res: String = zap
        .zap
        .lock()
        .unwrap()
        .list()
        .map(|(k, v)| format!("{}: {}", k, v,))
        .collect();
    return res;
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ZapState {
                zap: Mutex::new(Zap::new()),
            }))
            .service(get)
            .service(set)
            .service(has)
            .service(delete)
            .service(list)
            .service(ping)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
