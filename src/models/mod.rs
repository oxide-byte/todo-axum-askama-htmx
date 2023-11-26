use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created: Instant
}

#[derive(Clone, Debug)]
pub struct TodoList {
    todos: Vec<Todo>
}

impl TodoList {
    pub fn get_all(&self) -> Vec<Todo> {
        self.todos.clone()
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn remove(&mut self, id:String){
        self.todos.pop();
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self {
            todos : Vec::new()
        }
    }
}