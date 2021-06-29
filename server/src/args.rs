use clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum BlocksSource {
        RPC,
        DB,
    }
}

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum TxSource {
        NONE,
        RPC,
        DB,
    }
}

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum AddrSource {
        NONE,
        BLOCKCHAINCOM,
        BLOCKCHAIR,
        BLOCKCIPHER,
    }
}

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
    /// Source of address data
    #[structopt(long, default_value = "NONE", possible_values = &AddrSource::variants(), env = "SOURCE_ADDR")]
    pub addr: AddrSource,
    /// Source of transaction data
    #[structopt(long, default_value = "RPC", possible_values = &TxSource::variants(), case_insensitive = true,  env = "SOURCE_TX")]
    pub tx: TxSource,
    /// Source of blocks data
    #[structopt(long, default_value = "RPC", possible_values = &BlocksSource::variants(), case_insensitive = true, env = "SOURCE_BLOCKS")]
    pub blocks: BlocksSource,
    /// Bitcoin RPC address
    #[structopt(long, default_value = "http://localhost:8332/", env = "RPC_ADDR")]
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
