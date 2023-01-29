use serde::Serialize;
use tera::{Context, Error};

pub fn get_context(name: String, age: i32, is_admin: bool) -> Context {
    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("age", &age);
    context.insert("is_admin", &is_admin);
    context
}

#[derive(Serialize)]
pub struct User {
    name: String,
    age: i32,
    is_admin: bool,
}

impl User {
    pub fn new(name: String, age: i32, is_admin: bool) -> Self {
        Self {
            name,
            age,
            is_admin,
        }
    }

    pub fn get_context(&self) -> Result<Context, Error> {
        Context::from_serialize(&self)
    }
}
