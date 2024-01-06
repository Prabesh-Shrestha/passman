use passman::{cmd, crypto, manager, utils};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let password_manager = manager::PasswordManager::new().await?;
    password_manager.hash_password("hello world");
    Ok(())
}
