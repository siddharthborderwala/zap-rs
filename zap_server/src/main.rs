use actix::prelude::*;
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

mod state;

type ZapAddr = Addr<state::ZapActor>;

#[get("/k/get/{key}")]
async fn get(zap: web::Data<ZapAddr>, key: web::Path<String>) -> impl Responder {
    zap.send(state::Get(key.to_string()))
        .await
        .map(|res| HttpResponse::Ok().body(res))
        .unwrap_or_else(|err| HttpResponse::InternalServerError().body(err.to_string()))
}

#[post("/k/set/{key}")]
async fn set(zap: web::Data<ZapAddr>, key: web::Path<String>, req_body: String) -> impl Responder {
    zap.send(state::Set(key.to_string(), req_body))
        .await
        .map(|_| HttpResponse::Ok().finish())
        .unwrap_or_else(|err| HttpResponse::InternalServerError().body(err.to_string()))
}

#[post("/k/delete/{key}")]
async fn delete(zap: web::Data<ZapAddr>, key: web::Path<String>) -> impl Responder {
    zap.send(state::Delete(key.to_string()))
        .await
        .map(|_| HttpResponse::Ok().finish())
        .unwrap_or_else(|err| HttpResponse::InternalServerError().body(err.to_string()))
}

#[get("/k/has/{key}")]
async fn has(zap: web::Data<ZapAddr>, key: web::Path<String>) -> impl Responder {
    zap.send(state::Has(key.to_string()))
        .await
        .map(|res| HttpResponse::Ok().body(res.to_string()))
        .unwrap_or_else(|err| HttpResponse::InternalServerError().body(err.to_string()))
}

#[get("/k/list")]
async fn list(zap: web::Data<ZapAddr>) -> impl Responder {
    zap.send(state::List)
        .await
        .map(|res| HttpResponse::Ok().body(res))
        .unwrap_or_else(|err| HttpResponse::InternalServerError().body(err.to_string()))
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = Env::new().default_filter_or("info");
    env_logger::init_from_env(env);
    let zapper = web::Data::new(state::ZapActor::new().start());

    HttpServer::new(move || {
        App::new()
            .app_data(zapper.clone())
            .service(get)
            .service(set)
            .service(has)
            .service(delete)
            .service(list)
            .service(ping)
            .default_service(web::to(|| HttpResponse::NotFound()))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_ping() {
        let mut app = test::init_service(
            App::new()
                .service(ping)
                .default_service(web::to(|| HttpResponse::NotFound()))
                .wrap(Logger::default()),
        )
        .await;
        let req = test::TestRequest::get().uri("/ping").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let body = test::read_body(resp).await;
        assert_eq!(body, "pong");
    }

    #[actix_web::test]
    async fn test_get_set() {
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(state::ZapActor::new().start()))
                .service(get)
                .service(set)
                .default_service(web::to(|| HttpResponse::NotFound()))
                .wrap(Logger::default()),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/k/set/foo")
            .set_payload("bar")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let req = test::TestRequest::get().uri("/k/get/foo").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let body = test::read_body(resp).await;
        assert_eq!(body, "bar");
    }

    #[actix_web::test]
    async fn test_has() {
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(state::ZapActor::new().start()))
                .service(has)
                .service(set)
                .default_service(web::to(|| HttpResponse::NotFound()))
                .wrap(Logger::default()),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/k/set/foo")
            .set_payload("bar")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let req = test::TestRequest::get().uri("/k/has/foo").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let body = test::read_body(resp).await;
        assert_eq!(body, "true");
    }

    #[actix_web::test]
    async fn test_delete() {
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(state::ZapActor::new().start()))
                .service(delete)
                .service(set)
                .service(has)
                .default_service(web::to(|| HttpResponse::NotFound()))
                .wrap(Logger::default()),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/k/set/foo")
            .set_payload("bar")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let req = test::TestRequest::get().uri("/k/has/foo").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let body = test::read_body(resp).await;
        assert_eq!(body, "true");
        let req = test::TestRequest::post().uri("/k/delete/foo").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let req = test::TestRequest::get().uri("/k/has/foo").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);
        let body = test::read_body(resp).await;
        assert_eq!(body, "false");
    }
}
