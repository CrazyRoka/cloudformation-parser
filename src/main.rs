use anyhow::Result;
use cfn_validator::{self, Template};
use std::fs;

fn main() -> Result<()> {
    let code = fs::read_to_string("./template.json")?;
    let result: Template = serde_json::from_str(&code)?;
    println!("{:?}", result);

    Ok(())
}
