pub mod btc;
// use sqlx::postgres::PgPoolOptions;

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = btc::new();
    let info = client.get_chain_info()?;
    println!("max block: {} {}", info.blocks, info.bestblockhash);

    let block = client.get_block(info.bestblockhash)?;
    println!("{} transactions", block.tx.len());

    // Create a connection pool
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://postgres:password@localhost/btcexplorer").await?;

    // // Make a simple query to return the given parameter
    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&pool).await?;
    // assert_eq!(row.0, 150);
    Ok(())
}
