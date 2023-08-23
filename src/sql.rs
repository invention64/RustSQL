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

        // format host and database since toml returns a string with quotes
        let host = secrets["host"].to_string().replace("\"","");
        let database = secrets["database"].to_string().replace("\"","");
        // also need to switch from double to single quotes to not cause issues with sql
        let user = secrets["user"].to_string().replace("\"","");
        let password = secrets["password"].to_string().replace("\"","");

        // build connection string
        let output: String = format!("mysql://{user}:{password}@{host}/{database}", user=user, password=password, host=host, database=database);

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