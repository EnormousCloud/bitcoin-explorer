use crate::btc;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::pool::PoolConnection;
use sqlx::Acquire;
use sqlx::Postgres;

pub async fn max_final_height(conn: &mut PoolConnection<Postgres>) -> Result<i32, anyhow::Error> {
    let row: (i32,) = sqlx::query_as("SELECT COALESCE(MAX(blockheight),0) FROM final_blocks")
        .fetch_one(conn)
        .await?;
    Ok(row.0)
}

pub async fn persist(
    conn: &mut PoolConnection<Postgres>,
    block: &btc::BlockInfoCombined,
    with_index: bool,
) -> Result<(), anyhow::Error> {
    let start = std::time::Instant::now();

    let height = block.stats.height;
    let hash = block.stats.blockhash.as_str();
    let hashb = hex::decode(hash)?;

    let mut tx = conn.begin().await?;
    // save header
    sqlx::query(
        "INSERT INTO longest_chain (blockheight, blockhash) \
            VALUES ($1, $2) \
            ON CONFLICT (blockheight) \
            DO UPDATE SET blockhash = EXCLUDED.blockhash",
    )
    .bind(height)
    .bind(hashb.clone())
    .execute(&mut tx)
    .await?;

    let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(block.info.time, 0), Utc);
    let mut txindex: i32 = 0;
    for tptr in block.info.tx.iter() {
        let t = tptr.clone();
        let txb = hex::decode(t.hash)?;
        let total: f32 = t.vout.iter().map(|x| x.value).sum();

        let on_conflict = if with_index {
            "ON CONFLICT (txhash) DO NOTHING"
        } else {
            ""
        };
        let sql_tx = [
            "INSERT INTO final_tx ( \
                txhash, txindex, blockhash, blockheight, sent_btc, fee_btc
            ) VALUES ($1, $2, $3, $4, $5, $6)",
            on_conflict,
        ];
        sqlx::query(sql_tx.join(" ").as_str())
            .bind(txb.clone())
            .bind(txindex)
            .bind(hashb.clone())
            .bind(height)
            .bind(total)
            .bind(0_f64)
            .execute(&mut tx)
            .await?;
        txindex += 1
    }
    let on_conflict = if with_index {
        "ON CONFLICT (blockhash) DO NOTHING"
    } else {
        ""
    };

    let sql_block = [
        "INSERT INTO final_blocks ( \
            blockheight,blockhash,tm,avgfee,avgfeerate,
            avgtxsize,ins,maxfee,maxfeerate,
            maxtxsize,medianfee,mediantxsize,minfee,minfeerate,
            mintxsize,outs,subsidy,swtotal_size,swtotal_weight,
            swtxs,total_out,total_size,total_weight,totalfee,
            txs,utxo_increase,utxo_size_inc
        ) \
        VALUES ( \
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, \
            $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, \
            $21, $22, $23, $24, $25, $26, $27
        )",
        on_conflict,
    ];
    // save block
    sqlx::query(sql_block.join(" ").as_str())
        .bind(height)
        .bind(hashb.clone())
        .bind(dt)
        .bind(block.stats.avgfee)
        .bind(block.stats.avgfeerate)
        .bind(block.stats.avgtxsize as i64)
        .bind(block.stats.ins)
        .bind(block.stats.maxfee as i64)
        .bind(block.stats.maxfeerate as i64)
        .bind(block.stats.maxtxsize)
        .bind(block.stats.medianfee)
        .bind(block.stats.mediantxsize)
        .bind(block.stats.minfee as i64)
        .bind(block.stats.minfeerate as i64)
        .bind(block.stats.mintxsize)
        .bind(block.stats.outs)
        .bind(block.stats.subsidy as i64)
        .bind(block.stats.swtotal_size)
        .bind(block.stats.swtotal_weight as i64)
        .bind(block.stats.swtxs)
        .bind(block.stats.total_out as i64)
        .bind(block.stats.total_size as i64)
        .bind(block.stats.total_weight as i64)
        .bind(block.stats.totalfee as i64)
        .bind(block.stats.txs)
        .bind(block.stats.utxo_increase)
        .bind(block.stats.utxo_size_inc)
        .execute(&mut tx)
        .await?;

    tx.commit().await?;

    println!(
        "saved block {}: {} transactions at {} took {:?}",
        hash,
        block.stats.txs,
        block.stats.height,
        start.elapsed(),
    );
    Ok(())
}
