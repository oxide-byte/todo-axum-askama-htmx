use askama::Template;
use serde::Deserialize;
use crate::models::{Todo};

#[derive(Debug, Deserialize)]
pub struct TodoCreateRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct TodoEditRequest {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Template)]
#[template(path = "todo_list.html")]
pub struct TodoListTemplate {
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "todo_create_modal.html")]
pub struct TodoCreateModalTemplate<> {
}

#[derive(Template)]
#[template(path = "todo_cancel_modal.html")]
pub struct TodoCancelModalTemplate<> {
}

#[derive(Template)]
#[template(path = "todo_edit_modal.html")]
pub struct TodoEditModalTemplate<> {
    pub todo: Todo
}