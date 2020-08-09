use webdriver_client::firefox::GeckoDriver;
use webdriver_client::messages::NewSessionCmd;
use webdriver_client::Driver;

use crate::command_sequence::CommandSequence;

use eyre::Result;

pub struct Browser {
    driver: GeckoDriver,
}

impl Browser {
    pub fn new() -> Result<Self> {
        let driver = GeckoDriver::spawn()?;
        Ok(Browser {
            driver,
        })
    }

    pub fn execute_command_sequence(self, command_sequence: CommandSequence) -> Result<()> {

        let session = self.driver.session(&NewSessionCmd::default())?;
        for command in command_sequence.iter() {
            command.execute(&session)?;
        }
        Ok(())
    }
}
