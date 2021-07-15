use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub struct DbConn {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DbConn {
    pub fn new(conn_string: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(conn_string);
        let pool = Pool::new(manager).unwrap();

        DbConn { pool }
    }

    pub fn get_conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().unwrap()
    }
}
