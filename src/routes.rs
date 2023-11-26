use std::time::Instant;
use axum::extract::State;
use axum::response::IntoResponse;
use crate::models::Todo;
use crate::SharedState;
use crate::templates::global_template::HtmlTemplate;
use crate::templates::index_template::IndexTemplate;
use crate::templates::todo_template::*;

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

pub async fn todo_list(State(shared_state): State<SharedState>) -> impl IntoResponse {
    let state = shared_state.read().unwrap();
    let todos = state.todos.get_all();
    let template = TodoListTemplate { todos };
    HtmlTemplate(template)
}

pub async fn todo_edit() -> impl IntoResponse {
    let template = TodoEditTemplate {};
    HtmlTemplate(template)
}

pub async fn todo_create_modal() -> impl IntoResponse {
    let template = TodoCreateModalTemplate {};
    HtmlTemplate(template)
}

pub async fn todo_edit_modal() -> impl IntoResponse {
    let template = TodoCreateModalTemplate {};
    HtmlTemplate(template)
}

pub async fn todo_cancel_modal() -> impl IntoResponse {
    let template = TodoCancelModalTemplate {};
    HtmlTemplate(template)
}

pub async fn todo_delete(State(shared_state): State<SharedState>, id:String) -> impl IntoResponse {
    let mut state = shared_state.write().unwrap();
    state.todos.remove("".to_string());

    let todos = state.todos.get_all();
    let template = TodoListTemplate { todos };
    HtmlTemplate(template)
}

pub async fn todo_create(State(shared_state): State<SharedState>) -> impl IntoResponse {

    let mut state = shared_state.write().unwrap();
    let todo = Todo{
        id: "1".to_string(),
        title: "1".to_string(),
        description: "1".to_string(),
        created: Instant::now(),
    };
    state.todos.add(todo);

    let todos = state.todos.get_all();
    let template = TodoListTemplate { todos };
    HtmlTemplate(template)
}