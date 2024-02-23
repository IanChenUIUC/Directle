#[macro_use]
extern crate rocket;

use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct GameInfo {
    secret_word: String,
    secter_word_vector: Vec<f32>,

    input_word: String,
    input_word_vector: Vec<f32>,

    output_word: String,
    output_word_vector: Vec<f32>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello?<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

#[get("/info/word?<word>")]
fn word_vector(word: String) -> String {
    let game_info = GameInfo {
        secret_word: "Amoung us".to_string(),
        secter_word_vector: vec![1.0, 2.0, 3.0],
        input_word: word,
        input_word_vector: vec![10.1, 111., -888., 1.],
        output_word: "Sussy baka".to_string(),
        output_word_vector: vec![818181., 10., 1.],
    };

    serde_json::to_string(&game_info).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, word_vector])
}
