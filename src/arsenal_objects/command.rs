use std::fmt::{self, Display};
use anyhow::Result;
use regex::Regex;
use toml::Value;

use crate::misc::inputs::IntelligentStringBuilder;


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
    pre: String,                    // Pre value before a <match>. Example: TEST<match>
    pub value: String,              // Litteral value, e.g. '<port=4444>'. This value is always set up.
    post: String,                   // Post value after a <match>. Example: <match>TEST
    is_input: bool,                 // If this value has to be an input
    default: Option<String>,        // If value is '<port=4444>' then default would be 4444. This would be the second value to be taken if not empty.
    pub modified: Option<String>,   // If value is overriden by user input then it is modified here. This would be the first value to be taken if not empty.
}

impl CommandArg {
    pub fn new(id: usize, arg: String) -> CommandArg {
        let mut cmd_arg = CommandArg { id, pre: "".to_string(), value: arg.clone(), post: "".to_string(), is_input: false, default: None, modified: None };

        // Character and regex for matching input arguments of commands
        let re = match Regex::new(r"(.*)<([a-zA-Z0-9-.:'!@#$%^&*\(\){}\[\]/|_=+]+)>(.*)") {
            Ok(r) => r,
            Err(_) => {
                return cmd_arg
            }
        };

        for (_, [pre, cap, post]) in re.captures_iter(&arg).map(|c| c.extract()) {
            let s = cap.split("|").map(|s| s.to_string()).collect::<Vec<String>>();
            if s.len() == 2 {  // Value default set
                cmd_arg.value = format!("<{}>", s.get(0).unwrap().clone());
                cmd_arg.default = Some(s.get(1).unwrap().clone());
            } else if s.len() == 1 {
                cmd_arg.value = format!("<{}>", s.get(0).unwrap().clone());
            }
            cmd_arg.is_input = true;
            cmd_arg.pre = pre.to_string();
            cmd_arg.post = post.to_string();
        }
        cmd_arg
    }

    pub fn copy(&self) -> String {
        match self.is_input {
            true => {
                if self.modified.is_some() {
                    format!("{}{}{}", self.pre, self.modified.clone().unwrap(), self.post)
                } else if self.default.is_some() {
                    format!("{}{}{}", self.pre, self.default.clone().unwrap(), self.post)
                } else {
                    format!("{}{}{}", self.pre, self.value, self.post)
                }
            },
            false => self.value.clone()
        }
    }
}

impl Display for CommandArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_input {
            // If input also add pre/post string to complete entire value
            if self.modified.is_some() {
                return write!(f, "({}) {}{}{} = {}{}{}", self.id, self.pre, self.value, self.post, self.pre, self.modified.clone().unwrap(), self.post)
            } else if self.default.is_some() {
                return write!(f, "({}) {}{}{} = {}{}{}", self.id, self.pre, self.value, self.post, self.pre, self.default.clone().unwrap(), self.post)
            } else {
                return write!(f, "({}) {}{}{} = ", self.id, self.pre, self.value, self.post);
            }    
        }

        write!(f, "{}", self.value)
    }
}

#[derive(Clone)]
pub struct Command {
    pub id: usize,
    pub name: String,  // Real name as the name in brackets `[command.xxx]` => xxx
    pub name_cmd: String,
    pub cmd_types: Vec<CommandType>,
    pub explanation: String,
    pub args: String,
    pub cmd_args: Vec<CommandArg>,
    pub examples: Vec<String>
}

impl Command {
    pub fn new(name: String, name_cmd: String, cmd_types: String, explanation: String, args: String, examples: Vec<String>) -> Self {
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

        let cmd_types_vector: Vec<CommandType> = cmd_types.split("|").into_iter()
            .map(|cmd_type| CommandType::from_str(&cmd_type))
            .collect();

        Command {
            id: unsafe { ID },
            name,
            name_cmd,
            cmd_types: cmd_types_vector,
            explanation,
            args,
            cmd_args,
            examples
        }
    }

    pub fn info(&self) -> String {
        format!(
            "Command:{}\n\
            TYPE:{}\n\
            Explanation:\n{}\n\
            \
            {} {}\n\
            \
            Examples:\n > {}",
            self.name_cmd,
            self.cmd_types.iter()
                .map(|cmd_type| format!("{:?}", cmd_type))
                .collect::<Vec<String>>().join(" "),
            self.explanation,
            self.name_cmd,
            self.copy_raw(),
            self.examples.join("\n > ")
        )
    }

    pub fn copy_raw(&self) -> String {
        let cmd = self.cmd_args.iter()
            .map(|arg| format!("{}{}{}", arg.pre, arg.value, arg.post))
            .collect::<Vec<String>>()
            .join(" ");

        format!("{} {}", self.name_cmd, cmd)
    }

    pub fn copy_raw_shifted(&self) -> String {
        let cmd = self.cmd_args.iter()
            .map(|arg| format!("{}{}{}", arg.pre, arg.value, arg.post))
            .collect::<Vec<String>>()
            .join(" ");

        format!("[{:<20}] {} {}", self.name, self.name_cmd, cmd)
    }

    pub fn copy_basic(&self) -> String {
        let cmd = self.cmd_args.iter()
            .map(|arg| arg.copy())
            .collect::<Vec<String>>()
            .join(" ");

        format!("{} {}", self.name_cmd, cmd)
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
        write!(f, "{}", self.copy_basic())
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
            let name = k_command.clone().replace("-", " ");
            let mut name_cmd = k_command.clone();
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
                    let mut isb = IntelligentStringBuilder::new(arg_value.to_string());
                    let val = isb.delete_first_quote().delete_last_quote().replace_backslash_quote_with_quote().build();

                    if arg_key == "examples" {
                        let Some(examples) = arg_value.as_array() else {
                            continue;
                        };
                        for example in examples.iter() {
                            let mut isb = IntelligentStringBuilder::new(example.to_string());
                            cmd_examples.push(isb.delete_first_quote().delete_last_quote().replace_backslash_quote_with_quote().build());
                        }
                    } else if arg_key == "name_exe"{
                        name_cmd = val;
                    } else if arg_key == "cmd_types"{ 
                        cmd_type = val;
                    } else if arg_key == "explanation"{ 
                        explanation = val;
                    } else if arg_key == "args" {
                        args = val;
                    }
                }
            }

            commands.push(Command::new(name, name_cmd, cmd_type, explanation, args, cmd_examples));
        }
    }

    return Ok(commands)
}