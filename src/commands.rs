use std::slice::Iter;

use eyre::Result;
pub enum Commands {
    Get(String)
}

pub struct CommandSequence {
    commands: Box<[Commands]>
}

pub struct CommandSequenceBuilder{
    commands : Vec<Commands>
}

impl CommandSequenceBuilder {
    pub fn new() -> Self {
        CommandSequenceBuilder{commands : vec!()}
    }

    pub fn append_command(&mut self, command: Commands) -> Result<()> {
        self.commands.push(command);
        Ok(())
    }

    pub fn build(self) -> CommandSequence{
        CommandSequence{commands: self.commands.into_boxed_slice()}
    }
}

impl CommandSequence { 
    pub fn iter(&self) -> Iter<Commands> {
        self.commands.iter()
    }
}