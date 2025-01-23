use sails_rs::prelude::*;

#[derive(Default)]
pub struct Service;

#[service]
impl Service {
    pub fn new() -> Self {
        Self
    }

    pub fn hello(&mut self) -> String {
        "Hello world!".to_string()
    }
}