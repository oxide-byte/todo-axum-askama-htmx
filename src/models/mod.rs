use chrono::{NaiveDateTime};

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created: NaiveDateTime
}

#[derive(Clone, Debug)]
pub struct TodoList {
    todos: Vec<Todo>
}

impl TodoList {
    pub fn get_all(&self) -> Vec<Todo> {
        self.todos.clone()
    }

    pub fn get_todo(&self, id:String) -> Todo {
        let index = self.todos.iter().position(|x| *x.id == id).unwrap();
        self.todos.get(index).unwrap().clone()
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn remove(&mut self, id:String){
        self.todos.retain(|x| *x.id != id);
    }

    pub fn update(&mut self, mut new_todo:Todo) {
        let index = self.todos.iter().position(|x| *x.id == new_todo.id).unwrap();
        let old = self.todos.get(index).unwrap();
        new_todo.created = old.created;
        let _ = std::mem::replace(&mut self.todos[index], new_todo);
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self {
            todos : Vec::new()
        }
    }
}