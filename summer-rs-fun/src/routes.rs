use crate::config::AppInfo;
use crate::task::{NewTask, Task, TaskService};
use summer_web::axum::Json;
use summer_web::axum::http::StatusCode;
use summer_web::error::{KnownWebError, Result};
use summer_web::extractor::{Component, Config, Path};
use summer_web::{delete, get, post, put};

fn not_found(id: u64) -> KnownWebError {
    KnownWebError::not_found(format!("task {id} not found"))
}

#[get("/")]
async fn index(Config(info): Config<AppInfo>) -> String {
    format!("{} - {}", info.greeting, info.name)
}

#[get("/health")]
async fn health() -> &'static str {
    "UP"
}

#[get("/tasks")]
async fn list_tasks(Component(service): Component<TaskService>) -> Json<Vec<Task>> {
    Json(service.list())
}

#[get("/tasks/{id}")]
async fn get_task(
    Component(service): Component<TaskService>,
    Path(id): Path<u64>,
) -> Result<Json<Task>> {
    Ok(Json(service.find(id).ok_or_else(|| not_found(id))?))
}

#[post("/tasks")]
async fn create_task(
    Component(service): Component<TaskService>,
    Json(new_task): Json<NewTask>,
) -> Result<(StatusCode, Json<Task>)> {
    let task = service
        .create(&new_task.title)
        .ok_or_else(|| KnownWebError::bad_request("title must not be blank"))?;
    Ok((StatusCode::CREATED, Json(task)))
}

#[put("/tasks/{id}/done")]
async fn complete_task(
    Component(service): Component<TaskService>,
    Path(id): Path<u64>,
) -> Result<Json<Task>> {
    Ok(Json(service.complete(id).ok_or_else(|| not_found(id))?))
}

#[delete("/tasks/{id}")]
async fn delete_task(
    Component(service): Component<TaskService>,
    Path(id): Path<u64>,
) -> Result<StatusCode> {
    if service.remove(id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(not_found(id))?
    }
}
