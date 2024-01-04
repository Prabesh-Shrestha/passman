mod cmd;
mod crypto;
mod manager;
mod utils;
use database::Database;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = utils::get_env();
    let url = env.get(&String::from("URL")).unwrap().to_string();

    let db = Database::new(url);

    println!("Hello, world!");
    Ok(())
}
