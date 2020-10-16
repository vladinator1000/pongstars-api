use dotenv::dotenv;
use std::env;


pub fn get_pool() -> () {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DATABASE_URL found"); // TODO: handle errors

    ()
}
