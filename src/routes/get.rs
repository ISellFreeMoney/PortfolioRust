use actix_web::{get, web, HttpResponse, Responder};
use diesel::RunQueryDsl;
use crate::DbPool;
use crate::schema::tasks::dsl::*;
use crate::types::{AppState, ErrNoId, Task};

#[get("/tasks")]
async fn get_tasks(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || tasks.load::<Task>(&conn))
        .await
        .map_err(|e| {
            eprintln!("Error loading tasks: {}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/tasks/{id}")]
pub async fn get_task(id: web::Path<String>, data: web::Data<AppState>) -> impl Responder{
    let task_id: String = id.into_inner();
    let tasks = data.tasks.lock().unwrap();
    let task_data = tasks.iter().find(|x| x.id == task_id);

    match task_data {
        None => {
            let response = ErrNoId{
                id: task_id,
                err: String::from("Book Not Found")
            };
            Err(response)
        },
        Some (book) => {
            let response = serde_json::to_string(book).unwrap();
            Ok(
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(response))
        }
    }
}