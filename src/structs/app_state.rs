use std::sync::Arc;

use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;

pub struct MySQLAppState {
    app_name: String,
    pool: Arc<Pool<MysqlConnectionManager>>,
}

impl MySQLAppState {
    pub fn new(app_name: String, pool: Arc<Pool<MysqlConnectionManager>>) -> Self {
        MySQLAppState {
            app_name,
            pool,
        }
    }
    pub fn get_app_name(&self) -> &str {
        &self.app_name
    }
    pub fn get_pool(&self) -> &Arc<Pool<MysqlConnectionManager>> {
        &self.pool
    }
}