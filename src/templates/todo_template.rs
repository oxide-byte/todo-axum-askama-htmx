use askama::Template;
use crate::models::{Todo};

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
#[template(path = "todo_edit.html")]
pub struct TodoEditTemplate<> {
}