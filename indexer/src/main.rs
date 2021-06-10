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

    let max_height: u32 = block::max_final_height(&mut conn).await? as u32;
    println!("recorded final height: {}", max_height);

    let mut height = max_height;
    loop {
        let hash = client.get_block_hash(height)?;
        println!("processing height {} hash {}", height, hash.as_str());
        let block = client.get_block(hash.as_str())?;
        block::persist(&mut conn, &block).await?;
        height = height + 1;
        if height > info.blocks {
            break;
        }
    }
    Ok(())
}
