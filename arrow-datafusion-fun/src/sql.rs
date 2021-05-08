use datafusion::prelude::*;
use arrow::util::pretty::print_batches;
use arrow::record_batch::RecordBatch;

pub async fn compute() -> datafusion::error::Result<()> {
    println!("SQL API: ");

    // register the table
    let mut ctx = ExecutionContext::new();
    ctx.register_csv("example", "example.csv", CsvReadOptions::new())?;
    // create a plan to run a SQL query
    let df = ctx.sql("SELECT a, MIN(b) FROM example GROUP BY a LIMIT 100")?;
    // execute and print results
    let results: Vec<RecordBatch> = df.collect().await?;
    print_batches(&results)?;

    Ok(())
}