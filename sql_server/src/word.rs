pub type F = f64;

use ormlite::model::*;
#[derive(Model, Debug)]
pub struct Word {
    pub id: i32,
    pub text: String,
    pub embedding: String,
}

impl Word {
    pub fn new(id: i32, text: String, embedding: Vec<F>) -> Self {
        Self {
            id,
            text,
            embedding: format!("{:?}", embedding),
        }
    }
}
