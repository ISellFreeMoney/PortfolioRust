use crate::routes::delete::delete_task;
use crate::routes::get::{get_task, get_tasks};
use crate::routes::post::post_task;
use crate::routes::put::update_task;
use actix_web::{web, App, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, SqliteConnection};

mod schema;
mod routes;
mod types;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABSE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_tasks)
            .service(get_task)
            .service(update_task)
            .service(post_task)
            .service(delete_task)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}