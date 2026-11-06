use ::sqlite::{sqlite, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = sqlite::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.run().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-sqlite operational");
    Ok(())
}
