use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "bitcoin-explorer", about = "Bitcoin Explorer")]
pub struct Args {
    /// Net listening address of HTTP server
    #[structopt(long, default_value = "0.0.0.0:8000", env = "LISTEN")]
    pub listen: String,
    /// Postgres Database connection URL
    #[structopt(
        long,
        default_value = "postgres://postgres:password@localhost/btcexplorer",
        env = "DATABASE_URL"
    )]
    pub database_url: String,
    #[structopt(long, default_value = "5", env = "DATABASE_MAX_CONN")]
    /// Postgres Database max connections
    pub database_conn: u32,
    /// Static folder to serve web client files
    #[structopt(long, default_value = "./dist", env = "STATIC_DIR")]
    pub static_dir: String,
    /// Source of address data: NONE, DB
    #[structopt(long, default_value = "NONE", env = "SOURCE_ADDR")]
    pub addr: String,
    /// Source of transaction data: NONE, DB, RPC
    #[structopt(long, default_value = "RPC", env = "SOURCE_TX")]
    pub tx: String,
    /// Source of blocks data: NONE, DB, RPC
    #[structopt(long, default_value = "RPC", env = "SOURCE_BLOCKS")]
    pub blocks: String,
    /// Bitcoin RPC address
    #[structopt(long, default_value = "localhost:8332", env = "RPC_ADDR")]
    pub rpc_addr: String,
    /// Bitcoin RPC user name
    #[structopt(long, default_value = "btcuser", env = "RPC_USERNAME")]
    pub rpc_username: String,
    /// Bitcoin RPC user password
    #[structopt(long, default_value = "", env = "RPC_PASSWORD")]
    pub rpc_password: String,
}

pub fn parse() -> anyhow::Result<Args> {
    let res = Args::from_args();
    tracing::info!("{:?}", res);
    Ok(res)
}
