mod sql;
mod dataframe;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    sql::compute().await?;
    dataframe::compute().await?;
    Ok(())
}