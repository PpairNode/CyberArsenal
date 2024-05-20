use std::fmt::{self, Display};
use anyhow::Result;
use regex::Regex;
use toml::Value;


#[derive(Debug, Clone)]
pub enum CommandType {
    PROGRAMMING,
    PENTEST,
    REVERSE,
    FORENSICS,
    CRYPTO,
    SYSADMIN,
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
            "sysadmin" => CommandType::SYSADMIN,
            "network" => CommandType::NETWORK,
            _ => CommandType::UNKNOWN
        }
    }
}

#[derive(Debug, Clone)]
// Structure used to represent modifications on a command arg
pub struct CommandArg {
    pub id: usize,                  // Used to know which argument is being modified
    value: String,                  // Litteral value, e.g. '<port=4444>'. This value is always set up.
    is_input: bool,                 // If this value has to be an input
    default: Option<String>,        // If value is '<port=4444>' then default would be 4444. This would be the second value to be taken if not empty.
    pub modified: Option<String>,   // If value is overriden by user input then it is modified here. This would be the first value to be taken if not empty.
}

impl CommandArg {
    pub fn new(id: usize, arg: String) -> CommandArg {
        let mut cmd_arg = CommandArg { id, value: arg.clone(), is_input: false, default: None, modified: None };

        let re = match Regex::new(r"<([a-zA-Z0-9=-_.]+)>") {
            Ok(r) => r,
            Err(_) => {
                return cmd_arg
            }
        };

        for (_, [cap]) in re.captures_iter(&arg).map(|c| c.extract()) {
            let s = cap.split("=").map(|s| s.to_string()).collect::<Vec<String>>();
            if s.len() == 2 {  // Value default set
                cmd_arg.value = format!("<{}>", s.get(0).unwrap().clone());
                cmd_arg.default = Some(s.get(1).unwrap().clone());
            } else if s.len() == 1 {
                cmd_arg.value = format!("<{}>", s.get(0).unwrap().clone());
            }
            cmd_arg.is_input = true;
        }
        cmd_arg
    }

    pub fn copy(self) -> String {
        match self.is_input {
            true => {
                if self.modified.is_some() {
                    self.modified.clone().unwrap()
                } else if self.default.is_some() {
                    self.default.clone().unwrap()
                } else {
                    self.value
                }
            },
            false => self.value    
        }
    }
}

impl Display for CommandArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_input {
            if self.modified.is_some() {
                return write!(f, "({}){} = {}", self.id, self.value, self.modified.clone().unwrap())
            } else if self.default.is_some() {
                return write!(f, "({}){} = {}", self.id, self.value, self.default.clone().unwrap())
            } else {
                return write!(f, "({}){} = ", self.id, self.value);
            }    
        }

        write!(f, "{}", self.value)
    }
}

#[derive(Clone)]
pub struct Command {
    pub id: usize,
    pub name: String,
    pub cmd_type: CommandType,
    pub explanation: String,
    pub args: String,
    pub cmd_args: Vec<CommandArg>,
    pub examples: Vec<String>
}

impl Command {
    pub fn new(name: String, cmd_type: String, explanation: String, args: String, examples: Vec<String>) -> Self {
        static mut ID: usize = 0;
        unsafe { ID = ID + 1 };

        let mut id = 0;
        let v = args.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        let cmd_args: Vec<CommandArg> = v.iter()
            .map(|s| {
                let cmd_arg = CommandArg::new(id, s.to_string());
                id += 1;
                cmd_arg
            })
            .collect();

        let cmd_type = CommandType::from_str(&cmd_type);
        Command {
            id: unsafe { ID },
            name,
            cmd_type,
            explanation,
            args,
            cmd_args,
            examples
        }
    }

    pub fn info(&self) -> String {
        format!(
            "Command:{}\n\
            TYPE:{:?}\n\
            Explanation:\n{}\n\
            \
            {} {}\n\
            \
            Examples:\n > {}",
            self.name,
            self.cmd_type,
            self.explanation,
            self.name,self.args,
            self.examples.join("\n > ")
        )
    }

    pub fn copy(&self) -> String {
        let cmd = self.cmd_args.iter()
            .map(|arg| arg.clone().copy())
            .collect::<Vec<String>>()
            .join(" ");

        format!("{} {}", self.name, cmd)
    }

    pub fn get_all_args(&self) -> &Vec<CommandArg> {
        &self.cmd_args
    }

    pub fn get_input_args(&self) -> Vec<CommandArg> {
        self.cmd_args.iter()
            .filter_map(|cmd_arg| if cmd_arg.is_input { Some(cmd_arg.clone()) } else { None })
            .collect()
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.copy())
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
            let mut name = k_command.clone();
            let mut cmd_type = "".to_string();
            let mut explanation = "".to_string();
            let mut args = "".to_string();
            let mut cmd_examples = vec![];

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
                            cmd_examples.push(example.to_string().replace("\"", ""));
                        }
                        // Remove `",[,]` from examples as we do not need them for the presentation
                        // tmp_command.examples.push(arg_value.to_string().replace("\"", "").replace("[", "").replace("]", ""));
                    } else if arg_key == "name_exe"{
                        name = arg_value.to_string().replace("\"", "");
                    } else if arg_key == "cmd_type"{ 
                        cmd_type = arg_value.to_string().replace("\"", "");
                    } else if arg_key == "explanation"{ 
                        explanation = arg_value.to_string().replace("\"", "");
                    } else if arg_key == "args" {
                        args = arg_value.to_string().replace("\"", "");
                    }
                }
            }

            commands.push(Command::new(name, cmd_type, explanation, args, cmd_examples));
        }
    }

    return Ok(commands)
}