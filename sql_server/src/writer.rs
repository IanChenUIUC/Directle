type F = f64;

#[derive(Debug, PartialEq, Clone)]
pub struct Word {
    word: String,
    embedding: Vec<F>,
}

impl Word {
    pub fn new(word: String, embedding: Vec<F>) -> Self {
        Self {
            word,
            embedding
        }
    }
}
