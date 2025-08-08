mod ex_if_else;
mod ex_option_unwrap;
mod ex_for_loops;
mod ex_traits;
mod ex_http;
mod ex_json_serde;
mod ex_shell_cmd;
mod ex_borrow_checker;
mod ex_unit_tests;
mod ex_collections;
mod ex_error_handling;
mod ex_functional;

#[tokio::main]
async fn main() {
    ex_if_else::run();
    ex_option_unwrap::run();
    ex_for_loops::run();
    ex_traits::run();
    ex_borrow_checker::run();
    ex_collections::run();
    ex_error_handling::run();
    ex_functional::run();
    ex_shell_cmd::run().unwrap();
    ex_unit_tests::run();
    ex_json_serde::run();
    ex_http::run().await.unwrap();
}
