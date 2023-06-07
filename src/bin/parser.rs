// SPDX-License-Identifier: Apache-2.0

use std::{
    fs,
    path::{Path, PathBuf},
};
use substrait::{
    parse::{Context, Parser},
    proto,
};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// Path to JSON file with plan input to parse
    input: PathBuf,
}

#[tracing::instrument]
fn parse(input: &Path) -> Result<(), anyhow::Error> {
    let input_file = fs::read_to_string(input)?;
    let plan = serde_json::from_str::<proto::Plan>(&input_file)?;
    let mut parser = Parser::default();
    parser.parse(plan)?;
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let Args { input } = <Args as clap::Parser>::parse();

    tracing_subscriber::fmt()
        .pretty()
        .with_file(false)
        .with_line_number(false)
        .without_time()
        .init();

    parse(&input)
}
