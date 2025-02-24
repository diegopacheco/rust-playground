use diesel::prelude::*;
use diesel::insert_into;

mod schema;
use schema::users;

use dotenvy::dotenv;
use std::env;

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser<'a> {
    name: &'a str,
    hair_color: Option<&'a str>,
}


#[derive(Queryable, Debug)]
struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}


fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect("Error connecting to database");

    let new_users = vec![
        NewUser { name: "Sean", hair_color: Some("Black") },
        NewUser { name: "Gordon", hair_color: None },
    ];

    insert_into(users::table)
        .values(&new_users)
        .execute(&mut connection)
        .expect("Error inserting users");
    
    let results = users::table
        .select(users::all_columns)
        .load::<User>(&mut connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user);
    }

    println!("Done.");
}