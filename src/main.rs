
#![allow(unused_variables, unused_imports, dead_code)]
use passman::{cmd, crypto, manager, utils};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let password_manager = manager::PasswordManager::new().await?;
    Ok(())
}
