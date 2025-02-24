use crate::model::Cat;
use std::sync::Mutex;

pub struct AppState {
    pub cats: Mutex<Vec<Cat>>,
}