pub mod driver {
    use toml::Table;
    use std::fs::File;
    use std::io::prelude::*;
    use mysql::{Pool, PooledConn};
    // WARNING: This file depends on a Secrets.toml with the following structure
    /*
    [connection]
    user = ""
    password = ""
    database = ""
    host = ""
     */

    // This function takes the info from a Secrets.toml file and converts it into a connection string
    pub fn build_conn_string() -> String {
        // open Secrets.toml
        let mut file_result = File::open("Secrets.toml");
        let mut file = match file_result {
            Ok(v) => v,
            Err(e) => panic!("Secrets.toml file unreadable! Unable to form connection string."),
        };
        let mut file_content = String::new();
        let mut result = file.read_to_string(&mut file_content);
        assert!(!file_content.is_empty());
        // parse toml
        let secrets = file_content.parse::<Table>().unwrap();

        // build connection string
        let output: String = format!("mysql://{}:{}@{}/{}", secrets["user"], secrets["password"], secrets["host"], secrets["database"]);

        return output;
    }

    pub fn open_connection() -> PooledConn {
        let url = build_conn_string();
        let pool_result = Pool::new(&*url);
        let pool = match pool_result {
            Ok(p)=> p,
            Err(e)=> panic!("Failed to create pool: {}",e)
        };
        let conn_result = pool.get_conn();
        let mut conn = match conn_result {
          Ok(c) => c,
            Err(e) => panic!("Unable to open sql connection: {}", e)
        };
        return conn;
    }

}