use mysql::PooledConn;
use mysql::Pool;
use dotenv::dotenv;
use std::env;

/**
 * Établit une connexion à la base de données.
 *
 * @return Une connexion à la base de données (`PooledConn`).
 */
pub fn get_conn() -> PooledConn {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::new(database_url).expect("Failed to create pool.");
    pool.get_conn().expect("Failed to get connection.")
}