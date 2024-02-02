// SPDX-License-Identifier: Apache-2.0

use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
};
use substrait::{
    parse::{Context as _, Parser},
    proto,
};
use tracing::{Level, Metadata, Subscriber};
use tracing_subscriber::{layer::Context, prelude::*, Layer};

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

    /// Only show the first error.
    #[derive(Default)]
    struct Filter(AtomicBool);

    impl<S: Subscriber> Layer<S> for Filter {
        fn enabled(&self, metadata: &Metadata<'_>, _ctx: Context<'_, S>) -> bool {
            if !self.0.load(Ordering::Relaxed) {
                if metadata.level() == &Level::ERROR {
                    self.0.store(true, Ordering::Relaxed);
                }
                true
            } else {
                false
            }
        }
    }

    let fmt = tracing_subscriber::fmt::layer()
        .pretty()
        .with_file(false)
        .with_line_number(false)
        .without_time();

    tracing_subscriber::registry()
        .with(Filter::default())
        .with(fmt)
        .init();

    parse(&input)
}
