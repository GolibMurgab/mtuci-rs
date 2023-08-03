use std::{fs::File, io::{BufReader, Error}};
use druid::{
    im::Vector, Data, Env, EventCtx, Lens,
    AppDelegate, Command, DelegateCtx, Handled, Selector, Target
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const SAVE: Selector = Selector::new("save");
pub const DELETE: Selector<Uuid> = Selector::new("delete");


#[derive(Clone, Data, Lens)]
pub struct Todo {
    new_todo: String,
    todos: Vector<TodoItem>,
}

impl Todo {
    pub fn add_item(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        if data.new_todo.trim() != "" {
            data.todos.push_front(TodoItem::new(data));
            data.new_todo = "".to_string();
            data.save_to_json().unwrap();
        }
    }

    pub fn save_to_json(&self) -> Result<(), Error> {
        let tds: Vec<TodoItem> = self.todos.iter().map(|item| item.to_owned()).collect();
        let serialized = serde_json::to_string_pretty(&tds)?;
        std::fs::write("todos.json", serialized)?;
        Ok(())
    }

    pub fn load_from_json() -> Self {
        let f = File::open("todos.json");

        match f {
            Ok(f) => {
                let reader = BufReader::new(f);
                let todos: Vec<TodoItem> = serde_json::from_reader(reader).unwrap_or(vec![]);
                Self {
                    todos: Vector::from(todos),
                    new_todo: String::new(),
                }
            }
            Err(_) => Self {
                todos: Vector::new(),
                new_todo: String::new(),
            },
        }
    }

    pub fn delete_todo(&mut self, id: &Uuid) {
        self.todos.retain(|item| &item.id != id);
        self.save_to_json().unwrap();
    }

    pub fn remove_done(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.todos.retain(|item| !item.done);
        data.save_to_json().unwrap();
    }
}

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct TodoItem {
    #[data(same_fn = "PartialEq::eq")]
    pub id: Uuid,
    pub done: bool,
    pub text: String,
}

impl TodoItem {
    pub fn new(td: &Todo) -> Self {
        let id = Uuid::new_v4();
        for i in &td.todos {
            if i.id == id {
                return TodoItem::new(td);
            }
        }
        Self {
            id,
            done: false,
            text: td.new_todo.clone().into(),
        }
    }

    pub fn delete_item(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        ctx.submit_command(DELETE.with(data.id));
    }
}

pub struct Delegate;

impl AppDelegate<Todo> for Delegate {
    fn command(&mut self, _ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut Todo, _env: &Env, ) -> Handled {
        if cmd.is(SAVE) {
            data.save_to_json().unwrap();
            Handled::Yes
        } else if let Some(id) = cmd.get(DELETE) {
            data.delete_todo(id);
            Handled::Yes
        } else {
            Handled::No
        }
    }
}