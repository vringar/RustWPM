use std::error::Error;

enum Commands {
    Get(url: String)
}

struct CommandSequence {
    commands: Box<Commands>
}

struct CommandSequenceBuilder{
    commands : Vec<Commands>
}

impl CommandSequenceBuilder {
    fn append_command(&mut self, command: Commands) -> Result<(), Error> {
        self.commands.push(command);
        Ok()
    }
    fn build(self) -> CommandSequence{
        CommandSequence{self.commands.into_boxed_slice()}
    }
}