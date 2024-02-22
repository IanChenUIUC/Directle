mod word;

use std::error::*;
use std::result::*;
use mysql::*;
use mysql::prelude::*;
use crate::word::*;


fn main() {
    let word1 = Word::new("hello".to_owned(), vec![1., 2., 3.]);
    let word2 = Word::new("world".to_owned(), vec![3., 2., 1.]);
}
