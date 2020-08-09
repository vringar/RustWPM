use webdriver_client::firefox::GeckoDriver;
use webdriver_client::messages::NewSessionCmd;
use webdriver_client::{Driver, DriverSession};

use crate::commands::{CommandSequence, Commands};

use std::{thread, time};

use eyre::Result;

pub struct Browser {
    command_sequence: CommandSequence,
}

impl Browser {
    pub fn new(command_sequence: CommandSequence) -> Self {
        Browser {
            command_sequence: command_sequence,
        }
    }

    pub fn execute_command_sequence(self) -> Result<()> {
        let driver = GeckoDriver::spawn().unwrap();
        let session = driver.session(&NewSessionCmd::default())?;
        for command in self.command_sequence.iter() {
            match command {
                Commands::Get(url) => self.get(url, &session)?,
            }
        }
        Ok(())
    }

    fn get(&self, url: &String, session: &DriverSession) -> Result<()> {
        session.go(&url)?;
        thread::sleep(time::Duration::from_secs(10));
        Ok(())
    }
}
