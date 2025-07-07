use actix_web::{delete, web, HttpResponse, Responder};
use diesel::RunQueryDsl;
use crate::DbPool;
use crate::schema::tasks::dsl::*;
use crate::types::{AppState, ErrNoId, Task};

#[delete("/books/{id}")]
async fn delete_task(id: web::Path<String>, data: web::Data<DbPool>) -> impl Responder {
    //todo
    
}