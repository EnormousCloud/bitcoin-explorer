pub mod block;
pub mod btc;

use sqlx::pool::PoolConnection;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = btc::new();
    let info = client.get_chain_info()?;
    println!("max block: {} {}", info.blocks, info.bestblockhash.as_str());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/btcexplorer")
        .await?;
    let mut conn: PoolConnection<Postgres> = pool.acquire().await.unwrap();
    let mut counter = 5;
    let mut hash = info.bestblockhash.clone();
    loop {
        let block = client.get_block(hash.as_str())?;
        block::persist(&mut conn, &block).await?;
        counter = counter - 1;
        if counter == 0 || block.info.height == 1 {
            break;
        }
        hash = block.info.previousblockhash;
    }
    Ok(())
}
