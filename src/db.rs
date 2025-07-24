use dotenvy::dotenv;
use mysql::*;
use std::env;

pub fn get_conn() -> Result<PooledConn, mysql::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL harus diset di .env");
    let opts = Opts::from_url(&database_url)?;
    let pool = Pool::new(opts)?;
    pool.get_conn()
}
