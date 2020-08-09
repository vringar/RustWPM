use std::slice::Iter;

use eyre::Result;
use webdriver_client::DriverSession;

pub trait Command {
    fn execute(&self, session: &DriverSession) -> Result<()>;
}

pub struct CommandSequence {
    commands: Box<[Box<dyn Command>]>,
}

pub struct CommandSequenceBuilder {
    commands: Vec<Box<dyn Command>>,
}

impl CommandSequenceBuilder {
    pub fn new() -> Self {
        CommandSequenceBuilder { commands: vec![] }
    }

    pub fn append_command(&mut self, command: impl Command + 'static) -> Result<()> {
        self.commands.push(Box::new(command));
        Ok(())
    }

    pub fn build(self) -> CommandSequence {
        CommandSequence {
            commands: self.commands.into_boxed_slice(),
        }
    }
}

impl CommandSequence {
    pub fn iter(&self) -> Iter<Box<dyn Command>> {
        self.commands.iter()
    }
}
