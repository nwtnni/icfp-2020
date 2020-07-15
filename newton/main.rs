fn main() -> anyhow::Result<()> {
    env_logger::init();

    let _ = icfp::Client::new()?;

    Ok(())
}
