use std::{fs, path::PathBuf};
use substrait::{
    parse::{parser::Parser, Context},
    proto,
};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// Path to JSON file with plan input to parse
    input: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let Args { input } = <Args as clap::Parser>::parse();

    let input_file = fs::read_to_string(input)?;
    let plan = serde_json::from_str::<proto::Plan>(&input_file)?;

    let mut parser = Parser::default();
    parser.parse(plan)?;

    Ok(())
}
