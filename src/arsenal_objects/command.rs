use std::fmt::{self, Display};
use anyhow::Result;
use toml::Value;


#[derive(Debug, Clone)]
pub enum CommandType {
    PROGRAMMING,
    PENTEST,
    REVERSE,
    FORENSICS,
    CRYPTO,
    ADMINSYS,
    NETWORK,
    UNKNOWN,
}

impl CommandType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "programming" | "" => CommandType::PROGRAMMING,
            "reverse" => CommandType::REVERSE,
            "forensics" => CommandType::FORENSICS,
            "pentest" => CommandType::PENTEST,
            "crypto" => CommandType::CRYPTO,
            "adminsys" => CommandType::ADMINSYS,
            "network" => CommandType::NETWORK,
            _ => CommandType::UNKNOWN
        }
    }
}

#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub cmd_type: CommandType,
    pub explanation: String,
    pub args: String,
    pub examples: Vec<String>
}

impl Command {
    pub fn new(name: String, cmd_type: String, explanation: String, args: String, examples: Vec<String>) -> Self {

        let cmd_type = CommandType::from_str(&cmd_type);
        Command {
            name,
            cmd_type,
            explanation,
            args,
            examples
        }
    }

    pub fn info(&self) -> String {
        // let s1 = vec!["check", "check2"].join(" \n ");
        format!(
            "Command: {}\n\
            Type: {:?}\n\
            Explanation: {}\n\
            \
            \n\
            Examples:\n > {}",
            self.name,
            self.cmd_type,
            self.explanation,
            self.examples.join("\n > ")
        )
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, self.args)
    }
}

pub fn load_values_into_commands(value: Value) -> Result<Vec<Command>> {
    let mut commands: Vec<Command> = vec![];

    let Some(table) = value.as_table() else {
        anyhow::bail!("Value as table error");
    };
    let Some(commands_value) = table.get("command") else {
        anyhow::bail!("Value does not contain command!");
    };

    for elt_commands in commands_value.as_table().iter() {
        for k_command in elt_commands.keys() {
            let mut tmp_command = Command::new(k_command.to_string(), "unknown".to_string(), "".to_string(), "".to_string(), vec![]);

            let v_args = elt_commands.get(k_command).unwrap();
            let args_value = v_args.as_table();
            for args_map in args_value.iter() {
                for arg_key in args_map.keys() {
                    let arg_value = args_map.get(arg_key).unwrap();
                    // Check few basic values
                    if arg_key == "examples" {
                        let Some(examples) = arg_value.as_array() else {
                            continue;
                        };
                        for example in examples.iter() {
                            tmp_command.examples.push(example.to_string().replace("\"", ""));
                        }
                        // Remove `",[,]` from examples as we do not need them for the presentation
                        // tmp_command.examples.push(arg_value.to_string().replace("\"", "").replace("[", "").replace("]", ""));
                    } else if arg_key == "name_exe"{
                        tmp_command.name = arg_value.to_string().replace("\"", "");
                    } else if arg_key == "cmd_type"{ 
                        tmp_command.cmd_type = CommandType::from_str(&arg_value.to_string().replace("\"", ""));
                    } else if arg_key == "explanation"{ 
                        tmp_command.explanation = arg_value.to_string().replace("\"", "");
                    } else if arg_key == "args" {
                        tmp_command.args = arg_value.to_string().replace("\"", "");
                    }
                }
            }

            commands.push(tmp_command);
        }
    }

    return Ok(commands)
}