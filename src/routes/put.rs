use actix_web::{put, web, HttpResponse, Responder};
use crate::types::{AppState, ErrNoId, Task};

#[put("/tasks/{id}")]
async fn update_task(id: web::Path<String>, req: web::Json<Task>, data: web::Data<AppState>) -> impl Responder{
    let task_id = id.into_inner();
    let new_task = Task {
        id: String::from(&req.id),
        title: String::from(&req.title),
        description: if req.description.is_some() { req.description.clone() } else { None} ,
        completed: false,
    };
    let mut tasks = data.tasks.lock().unwrap();
    let id_index = tasks.iter().position(|x| x.id == task_id);

    match id_index {
        None => {
            let response = ErrNoId {
                id: task_id,
                err: String::from("No task with that id"),
            };
            Err(response)
        }
        Some(id) => {
            let response = serde_json::to_string(&new_task).unwrap();
            tasks[id] = new_task;
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(response))
        }
    }
}