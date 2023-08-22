pub mod driver {
    // WARNING: This file depends on a Secrets.toml with the following structure
    /*
    [connection]
    user = ""
    password = ""
    database = ""
     */

    // This function takes the info from a Secrets.toml file and converts it into a connection string
    pub fn build_conn_string() -> String {
        let mut output = "";
    }
}