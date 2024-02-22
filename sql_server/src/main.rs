mod word;


use ormlite::sqlite::SqliteConnection;
use ormlite::model::*;
use word::Word;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start by making a database connection.
    let mut conn = <SqliteConnection as ormlite::Connection>::connect(":memory:").await.unwrap();

    // You can insert the model directly.
    let mut hello =  Word::new(1, "hello".to_owned(), vec![1., 2., 3.]).insert(&mut conn).await?;

    println!("{:?}", hello);

    // After modifying the object, you can update all its fields.
    hello.embedding += "1";
    hello.update_all_fields(&mut conn).await?;

    // Query builder syntax closely follows SQL syntax, translated into chained function calls.
    let people = Word::select()
        .where_("id > ?").bind(50)
        .fetch_all(&mut conn).await?;

    println!("{:?}", people);
    Ok(())
}
