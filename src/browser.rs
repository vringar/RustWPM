use webdriver_client::firefox::GeckoDriver;
use webdriver_client::messages::NewSessionCmd;
use webdriver_client::Driver;

use crate::command_sequence::CommandSequence;

use eyre::Result;

pub struct Browser {
    command_sequence: CommandSequence,
}

impl Browser {
    pub fn new(command_sequence: CommandSequence) -> Self {
        Browser {
            command_sequence,
        }
    }

    pub fn execute_command_sequence(self) -> Result<()> {
        let driver = GeckoDriver::spawn().unwrap();
        let session = driver.session(&NewSessionCmd::default())?;
        for command in self.command_sequence.iter() {
            command.execute(&session)?;
        }
        Ok(())
    }
}
