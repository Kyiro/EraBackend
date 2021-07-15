use actix_web::{get, put, web, HttpResponse, Responder};

#[get("/api/cloudstorage/system")]
pub async fn system() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/api/cloudstorage/system/config")]
pub async fn system_config() -> impl Responder {
    HttpResponse::NoContent().json(Vec::<i8>::new())
}

#[get("/api/cloudstorage/system/{i}")]
pub async fn system_file() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/api/cloudstorage/user/{i}")]
pub async fn user() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/api/cloudstorage/user/{i}/{f}")]
pub async fn user_file() -> impl Responder {
    HttpResponse::Ok()
}

#[put("/api/cloudstorage/user/{i}/{f}")]
pub async fn put_user_file(_: web::Bytes) -> impl Responder {
    HttpResponse::Ok()
}
