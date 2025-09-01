use zdiff::config::DiffConfig;

fn main() -> anyhow::Result<()> {
    let content = include_str!("../fixtures/test.yaml");
    let config = DiffConfig::from_yaml(content)?;

    println!("{:?}", config);
    Ok(())
}
