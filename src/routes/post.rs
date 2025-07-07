use actix_web::{post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::types::{AppState, NewTask, Task};

#[post("/tasks")]
pub async fn post_task(new_task: web::Json<NewTask>, data: web::Data<AppState>) -> impl Responder{
    println!("{:?}", new_task);
    let task = Task {
        id: Uuid::new_v4().to_string(),
        title: String::from(&new_task.title),
        description: new_task.description.clone(),
        completed: false,
    };
    let response = serde_json::to_string(&task).unwrap();
    let mut tasks = data.tasks.lock().unwrap();
    tasks.push(task);

    HttpResponse::Created()
        .content_type("application/json")
        .body(response)
}