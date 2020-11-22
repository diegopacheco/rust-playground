use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use reqwest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug",
)]
pub struct BookQuery;

async fn perform_my_query(variables: book_query::Variables) -> Result<(), Box<dyn Error>> {
    let request_body = BookQuery::build_query(variables);

    let client = reqwest::blocking::Client::builder()
        .user_agent("graphql-rust/0.9.0")
        .build()?;

    let res = client
        .post("http://localhost:8080/graphql")
        .json(&request_body)
        .send()?;

    let response_body: Response<book_query::ResponseData> = res.json()?;
    println!("{:#?}", response_body);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let q = book_query::Variables {
        book_id: Option::from("book-1".to_owned()),
   };
   let result = perform_my_query(q).await;
   println!("{:#?}", result);
   Ok(())
}