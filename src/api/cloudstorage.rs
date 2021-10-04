use actix_web::{get, put, web, HttpResponse, Responder};
use crate::structs::cloudstorage::*;
use crate::files::{get_cloudstorage, get_cloudstorage_file};

#[get("/api/cloudstorage/system")]
pub async fn system() -> impl Responder {
    let cloudstorage = get_cloudstorage();
    let mut entries = Vec::<SystemEntry>::new();
    
    for (name, data) in cloudstorage {
        entries.push(SystemEntry::new(name, data));
    }
    
    HttpResponse::Ok().json(entries)
}

#[get("/api/cloudstorage/system/config")]
pub async fn system_config() -> impl Responder {
    HttpResponse::NoContent().json(Vec::<i8>::new())
}

#[get("/api/cloudstorage/system/{i}")]
pub async fn system_file(
    file: web::Path<String>
) -> impl Responder {
    let file = file.into_inner();
    
    HttpResponse::Ok()
        .append_header(("content-type", "application/octet-stream"))
        .body(match get_cloudstorage_file(file) {
            Some(data) => data,
            None => return HttpResponse::NotFound().into()
        })
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
