use std::{env, ops::Deref};

use actix_cors::Cors;
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post, web, App, HttpResponse, HttpServer, Responder,
};
use backend::{
    create_new_task, get_five_tasks, get_pool, make_new_task, models::Task, schema::tasks, TaskJson,
};
use diesel::{Connection, PgConnection};
use serde::Deserialize;

type DbPool = deadpool_postgres::Pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_pool();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(hello)
            .service(get_tasks_route)
            .service(new_task_route)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/getTasks")]
async fn get_tasks_route(data: web::Data<DbPool>) -> impl Responder {
    let results = get_five_tasks(data)
        .await
        .expect("should have gotten results");

    let result = HttpResponse::Ok().json(results);
    println!("{:?}", result.body());
    result
}

#[post("/newtask")]
async fn new_task_route(data: web::Data<DbPool>, json: web::Json<TaskJson>) -> impl Responder {
    let _ = make_new_task(data, json.into_inner())
        .await
        .expect("could not make post");

    "hello"
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
