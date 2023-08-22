mod sql;
use crate::sql::driver;

fn main() {
    println!("{}", driver::build_conn_string());
    driver::open_connection();
}
