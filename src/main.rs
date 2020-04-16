
use log::debug;
use simple_logger;
use webdriver_client::Error;

mod commands;
use commands::*;

mod browser;

fn main() -> Result<(), Error> {
    simple_logger::init().unwrap();
    debug!("First log");
    let sequence = test().unwrap();
    let browser = browser::Browser::new(sequence);
    browser.execute_command_sequence()?;
    Ok(())
}

fn test() -> Result<CommandSequence,()> {
    let mut builder = CommandSequenceBuilder::new();
    let command = Commands::Get("http://asd.com".to_owned());
    builder.append_command(command)?;
    let cs = builder.build();
    Ok(cs)
}