use std::fmt::{self, Display};
use anyhow::Result;
use toml::Value;


#[derive(Debug)]
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

pub struct Command {
    pub name: String,
    pub cmd_type: CommandType,
    pub args: Vec<(String, String)>,
    pub examples: Vec<String>
}

impl Command {
    pub fn new(name: String, cmd_type: String, args: Vec<(String, String)>, examples: Vec<String>) -> Self {

        let cmd_type = CommandType::from_str(&cmd_type);
        Command {
            name,
            cmd_type,
            args,
            examples
        }
    }

    pub fn info(&self) -> String {
        format!(
            "Command: {}\n\
            Type: {:?}\n\
            \
            \n\
            Examples:\n\
            | {}",
            self.name,
            self.cmd_type,
            self.examples.join("\n| ")
        )
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: Vec<String> = self.args.iter()
            .map(|(_, s)| s.clone())
            .collect();
        write!(f, "{} {}", self.name, s.join(" "))
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
            let mut tmp_command = Command::new(k_command.to_string(), "unknown".to_string(), vec![], vec![]);

            let v_args = elt_commands.get(k_command).unwrap();
            let args_value = v_args.as_table();
            for args_map in args_value.iter() {
                for arg in args_map.keys() {
                    let arg_value = args_map.get(arg).unwrap();
                    // Check few basic values
                    if arg == "examples" {
                        // Remove `",[,]` from examples as we do not need them for the presentation
                        tmp_command.examples.push(arg_value.to_string().replace("\"", "").replace("[", "").replace("]", ""));
                    } else if arg == "name_exe"{ 
                        tmp_command.name = arg_value.to_string().replace("\"", "");
                    } else if arg == "cmd_type"{ 
                        tmp_command.cmd_type = CommandType::from_str(&arg_value.to_string().replace("\"", ""));
                    } else {
                        tmp_command.args.push((arg.clone().replace("\"", ""), arg_value.to_string().replace("\"", "")))
                    }
                }
            }

            commands.push(tmp_command);
        }
    }

    return Ok(commands)
}