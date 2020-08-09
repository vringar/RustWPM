use crate::command_sequence::Command;
use std::{thread, time};
use webdriver_client::DriverSession;

use eyre::Result;
pub struct Get {
    pub url: String,
}

impl Command for Get {
    fn execute(&self, session: &DriverSession) -> Result<()> {
        session.go(&self.url)?;
        thread::sleep(time::Duration::from_secs(10));
        Ok(())
    }
}
