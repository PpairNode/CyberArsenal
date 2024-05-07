use std::fmt::{self, Display};



pub enum CommandType {
    PROGRAMMING,
    PENTEST,
    REVERSE,
    FORENSICS
}

pub struct Command {
    pub name: String,
    pub values: Vec<String>,
    pub examples: Vec<String>
}

impl Command {
    pub fn new(name: String, values: Vec<String>, examples: Vec<String>) -> Self {
        Command {
            name,
            values,
            examples
        }
    }

    pub fn info(&self) -> String {
        format!(
            "Command: {}\n\
            \
            \n\
            Examples:\n\
            | {}",
            self.name,
            self.examples.join("\n| ")
        )
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, self.values.join(" "))
    }
}