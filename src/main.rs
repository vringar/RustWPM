use tracing::*;

mod command_sequence;
use command_sequence::{CommandSequence, CommandSequenceBuilder};
mod commands;
use commands::*;

mod browser;
use browser::Browser;

use eyre::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    info!("First log");
    let sequence = test().unwrap();
    let browser = Browser::new(sequence);
    browser.execute_command_sequence()?;
    Ok(())
}

fn test() -> Result<CommandSequence> {
    let mut builder = CommandSequenceBuilder::new();
    let command = Get {
        url: "http://example.com".to_owned(),
    };
    builder.append_command(command)?;
    let cs = builder.build();
    Ok(cs)
}
