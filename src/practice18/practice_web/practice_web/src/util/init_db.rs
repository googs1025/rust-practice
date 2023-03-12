use mysql::*;
static mut DB_POOL: Option<Pool> = None;

pub fn init_db(min: usize, max: usize) {
    let dns = "mysql://root:123456@localhost:3306/test";
    unsafe {
        DB_POOL = Some(Pool::new_manual(min, max, dns).unwrap());
    }
}

pub fn db() -> Result<PooledConn> {
    unsafe {
        match &DB_POOL {
            Some(pool) => pool.get_conn(),
            None => {
                panic!("init util panic!")
            }
        }
    }
}