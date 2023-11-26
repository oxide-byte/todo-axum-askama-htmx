mod templates;
mod routes;
mod models;

use std::sync::{Arc, RwLock};
use axum::{Router};
use axum::routing::{delete, get, post, put};
use tower_http::services::ServeDir;
use routes::*;
use crate::models::{TodoList};

#[derive(Debug)]
pub struct AppState {
    pub todos: TodoList
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            todos : TodoList::default()
        }
    }
}

pub type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() {
    let assets_path = std::env::current_dir().unwrap();
    let shared_state = SharedState::default();

    let app = Router::new()
        .route("/", get(index))
        .route("/todos", get(todo_list))
        .route("/todo-create-modal", get(todo_create_modal))
        .route("/todo-edit-modal", get(todo_edit_modal))
        .route("/todo-cancel-modal", get(todo_cancel_modal))
        .route("/todo-create", post(todo_create))
        .route("/todo-edit", put(todo_edit))
        .route("/todo-delete/{:id}", delete(todo_delete))
        .nest_service("/assets", ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),)
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}