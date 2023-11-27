use axum::extract::{Path, State};
use axum::Form;
use axum::response::IntoResponse;
use chrono::{ Utc};
use uuid::Uuid;
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

pub async fn todo_edit(State(shared_state): State<SharedState>, Form(todo_form): Form<TodoEditRequest>) -> impl IntoResponse {
    let mut state = shared_state.write().unwrap();

    let todo = Todo{
        id: todo_form.id,
        title: todo_form.title,
        description: todo_form.description,
        created: Utc::now().naive_utc()
    };

    state.todos.update(todo);

    let todos = state.todos.get_all();
    let template = TodoListTemplate { todos };
    HtmlTemplate(template)
}

pub async fn todo_create_modal() -> impl IntoResponse {
    let template = TodoCreateModalTemplate {};
    HtmlTemplate(template)
}

pub async fn todo_edit_modal(Path(id): Path<String>, State(shared_state): State<SharedState>) -> impl IntoResponse {
    let  state = shared_state.read().unwrap();
    let todo = state.todos.get_todo(id);
    let template = TodoEditModalTemplate {todo};
    HtmlTemplate(template)
}

pub async fn todo_cancel_modal() -> impl IntoResponse {
    let template = TodoCancelModalTemplate {};
    HtmlTemplate(template)
}

pub async fn todo_delete(Path(id): Path<String>, State(shared_state): State<SharedState>) -> impl IntoResponse {
    let mut state = shared_state.write().unwrap();
    state.todos.remove(id);

    let todos = state.todos.get_all();
    let template = TodoListTemplate { todos };
    HtmlTemplate(template)
}

pub async fn todo_create(State(shared_state): State<SharedState>, Form(todo_form): Form<TodoCreateRequest>) -> impl IntoResponse {

    let mut state = shared_state.write().unwrap();
    let todo = Todo{
        id: Uuid::new_v4().to_string(),
        title: todo_form.title,
        description: todo_form.description,
        created: Utc::now().naive_utc()
    };
    state.todos.add(todo);

    let todos = state.todos.get_all();
    let template = TodoListTemplate { todos };
    HtmlTemplate(template)
}