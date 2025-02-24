use diesel::prelude::*;
use diesel::insert_into;
use diesel::mysql::MysqlConnection;
mod schema;
use schema::person;
use dotenvy::dotenv;
use std::env;
use chrono::NaiveDateTime;

#[derive(Insertable)]
#[diesel(table_name = person)]
struct NewPerson<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = person)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
struct Person {
    id: i32,
    first_name: String,
    last_name: String,
    created_at: NaiveDateTime,
}
fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = MysqlConnection::establish(&database_url)
        .expect("Error connecting to database");

    let new_users = vec![
        NewPerson { first_name: "John", last_name: "Doe" },
        NewPerson { first_name: "Jane", last_name: "Doe" },
    ];

    insert_into(person::table)
        .values(&new_users)
        .execute(&mut connection)
        .expect("Error inserting users");
    
    let results = person::table
        .select(person::all_columns)
        .load::<Person>(&mut connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!(
            "ID: {}, First Name: {}, Last Name: {}, Created At: {}",
            user.id, user.first_name, user.last_name, user.created_at
        );
    }

    println!("Done.");
}