mod cmd;
mod crypto;
mod database;
mod utils;

fn main() {
    let env = utils::get_env();
    println!("{:#?}", env);

    println!("Hello, world!");
}
