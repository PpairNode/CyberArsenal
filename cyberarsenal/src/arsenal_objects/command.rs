use std::fmt::{self, Display};
use anyhow::Result;
use regex::Regex;
use rusqlite::Connection;
use tracing::{debug, info};



#[derive(Debug, Clone)]
pub enum CommandType {
    PROGRAMMING,
    PENTEST,
    REVERSE,
    FORENSICS,
    CRYPTO,
    SYSADMIN,
    NETWORK,
    NONE,
    UNKNOWN,
}

impl CommandType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "programming" => CommandType::PROGRAMMING,
            "reverse" => CommandType::REVERSE,
            "forensics" => CommandType::FORENSICS,
            "pentest" => CommandType::PENTEST,
            "crypto" => CommandType::CRYPTO,
            "sysadmin" => CommandType::SYSADMIN,
            "network" => CommandType::NETWORK,
            "" => CommandType::NONE,
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
    pub follow_char: Option<char>,          // If value is sticked to next, we shouldn't add a space. Example: <cur_match>/<next_match>
    pub is_input: bool,                 // If this value has to be an input
    default: Option<String>,        // If value is '<port=4444>' then default would be 4444. This would be the second value to be taken if not empty.
    pub modified: Option<String>,   // If value is overriden by user input then it is modified here. This would be the first value to be taken if not empty.
}

impl CommandArg {
    pub fn new(id: usize, args: String) -> Vec<CommandArg> {
        let mut cmd_arg_vec = Vec::<CommandArg>::new();

        // Character and regex for matching input arguments of commands
        // ([^:<>].*)(<[a-zA-Z0-9-.:'!@#$%\^&*\(\){}\[\]\/|_=+]+>)([^:<>].*) <= supposed to work flawlessely but no
        // (.*)<([a-zA-Z0-9-.:'!@#$%^&*\(\){}\[\]/|_=+]+)>(.*)
        // Maybe should extract this regex for loading => takes a while
        let re = match Regex::new(r"(.*)<([a-zA-Z0-9-.:'!@#$%^&*\(\){}\[\]/|_=+]+)>(.*)") {
            Ok(r) => r,
            Err(_) => {
                return vec![CommandArg { id, pre: "".to_string(), value: args, post: "".to_string(), follow_char: Some(' '), is_input: false, default: None, modified: None }]
            }
        };

        // Split because the regex which is supposed to work only works once
        let cmd_args = args.split_inclusive(">");
        let cmd_args_clone = cmd_args.clone();
        let final_post_command = match cmd_args_clone.last() {
            None => "".to_string(),
            Some(s) => {
                if !s.contains(">") {
                    s.to_string()
                } else {
                    "".to_string()
                }
            }
        };


        let mut tmp_id = id;

        let size_cmd_args = cmd_args.clone().count();
        let mut idx = 0;
        for s_full in cmd_args {
            for (_, [pre, cap, post]) in re.captures_iter(s_full).map(|c| c.extract()) {
                let s = cap.split("|").map(|s| s.to_string()).collect::<Vec<String>>();

                let mut cmd_arg = CommandArg { id, pre: "".to_string(), value: "".to_string(), post: "".to_string(), follow_char: Some(' '), is_input: false, default: None, modified: None };
                cmd_arg.id = tmp_id;
                if s.len() == 2 {  // Value default set
                    cmd_arg.value = format!("<{}>", s.get(0).unwrap().clone());
                    cmd_arg.default = Some(s.get(1).unwrap().clone());
                } else if s.len() == 1 {
                    cmd_arg.value = format!("<{}>", s.get(0).unwrap().clone());
                }
                cmd_arg.is_input = true;
                cmd_arg.pre = pre.to_string();
                cmd_arg.post = post.to_string();

                idx += 1;
                if idx < size_cmd_args {
                    cmd_arg.follow_char = None;
                }

                // Add last string
                if idx == size_cmd_args - 1 && !final_post_command.contains('>') {
                    cmd_arg.post = final_post_command.clone();
                }
                cmd_arg_vec.push(cmd_arg.clone());
                tmp_id += 1;
            }
        }

        if cmd_arg_vec.is_empty() {
            return vec![CommandArg { id, pre: "".to_string(), value: args, post: "".to_string(), follow_char: Some(' '), is_input: false, default: None, modified: None }]
        }

        cmd_arg_vec
    }

    pub fn copy(&self) -> String {
        match self.is_input {
            true => {
                if self.modified.is_some() {
                    format!("{}{}{}{}", self.pre, self.modified.clone().unwrap(), self.post, self.get_follow_char())
                } else if self.default.is_some() {
                    format!("{}{}{}{}", self.pre, self.default.clone().unwrap(), self.post, self.get_follow_char())
                } else {
                    format!("{}{}{}{}", self.pre, self.value, self.post, self.get_follow_char())
                }
            },
            false => format!("{}{}", self.value.clone(), self.get_follow_char())
        }
    }

    pub fn get_follow_char(&self) -> String {
        match self.follow_char {
            Some(c) => c.to_string(),
            None => "".to_string()
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
    pub name_exe: String,
    pub cmd_types: Vec<CommandType>,
    pub short_desc: String,
    pub details: String,
    pub args: String,
    pub cmd_args: Vec<CommandArg>,
    pub examples: Vec<String>
}

impl Command {
    pub fn new(id: usize, name: String, name_cmd: String, cmd_types: String, short_desc: String, details: String, args: String, examples: Vec<String>) -> Self {
        let mut cmd_id = 0;
        let v = args.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        let mut cmd_args: Vec<CommandArg> = vec![];
        for e in v {
            let mut cmd_arg_vec = CommandArg::new(cmd_id, e.to_string());
            cmd_id += cmd_arg_vec.len();  // CommandArg::new() returns a vector because it can have multiple fields to fill. Add len of vec to id 
            cmd_args.append(&mut cmd_arg_vec);
        }

        let cmd_types_vector: Vec<CommandType> = cmd_types.split("|").into_iter()
            .map(|cmd_type| CommandType::from_str(&cmd_type))
            .collect();

        Command {
            id,
            name,
            name_exe: name_cmd,
            cmd_types: cmd_types_vector,
            short_desc,
            details,
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
            Details:\n{}\n\
            \
            {} {}\n\
            \
            Examples:\n > {}",
            self.name_exe,
            self.cmd_types.iter()
                .map(|cmd_type| format!("{:?}", cmd_type))
                .collect::<Vec<String>>().join(" "),
            self.short_desc,
            self.details,
            self.name_exe,
            self.copy_raw(),
            self.examples.join("\n > ")
        )
    }

    pub fn short(&self) -> String {
        format!(
            "Command:{}\n\
            TYPE:{}\n\
            Explanation:\n{}\n\
            \
            {} {}\n",
            self.name_exe,
            self.cmd_types.iter()
                .map(|cmd_type| format!("{:?}", cmd_type))
                .collect::<Vec<String>>().join(" "),
            self.short_desc,
            self.name_exe,
            self.copy_raw()
        )
    }

    pub fn copy_raw(&self) -> String {
        let cmd = self.cmd_args.iter()
            .map(|arg| format!("{}{}{}{}", arg.pre, arg.value, arg.post, arg.get_follow_char()))
            .collect::<Vec<String>>()
            .join("");

        format!("{} {}", self.name_exe, cmd)
    }

    pub fn copy_raw_shifted(&self) -> String {
        let cmd = self.cmd_args.iter()
            .map(|arg| format!("{}{}{}{}", arg.pre, arg.value, arg.post, arg.get_follow_char()))
            .collect::<Vec<String>>()
            .join("");

        format!("[{:<20}] {} {}", self.name, self.name_exe, cmd)
    }

    pub fn copy_basic(&self) -> String {
        let cmd = self.cmd_args.iter()
            .map(|arg| arg.copy())
            .collect::<Vec<String>>()
            .join("");

        format!("{} {}", self.name_exe, cmd)
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

pub fn load_values_into_commands_from_db(name: &str) -> Result<Vec<Command>> {
    let conn = Connection::open(name)?;

    let mut cmd_statement = conn.prepare("SELECT id, name, name_exe, short_desc, details FROM commands;")?;
    let commands: Vec<Command> = cmd_statement.query_map([], |row| {
        let id: usize = row.get(0)?;
        let name = row.get(1)?;
        let name_exe = row.get(2)?;
        let short_desc = row.get(3)?;
        let details = row.get(4)?;

        debug!("ID: {id}");
        debug!("NAME: {name}");
        debug!("NAME_EXE: {name_exe}");
        debug!("SHORT_DESC: {short_desc}");
        debug!("DETAILS: {details}");

        debug!("Query: SELECT args FROM command_args WHERE COMMAND_ID={id};");
        let mut args_statment = conn.prepare(&format!("SELECT args FROM command_args WHERE command_id={id};"))?;
        let args: Vec<String> = args_statment.query_map([], |row| {
            debug!("Args row found: {row:?}");
            let args = row.get(0)?;
            debug!("Args extracted: {args:?}");
            Ok(args)
        })?.filter_map(Result::ok).collect();

        let args = match args.get(0) {
            Some(s) => s.clone(),
            None => String::new()
        };

        debug!("ARGS: {args}");

        debug!("Query: SELECT example FROM command_examples WHERE COMMAND_ID={id};");
        let mut examples_statment = conn.prepare(&format!("SELECT example FROM command_examples WHERE command_id={id};"))?;
        let examples: Vec<String> = examples_statment.query_map([], |row| {
            let examples = row.get(0)?;
            Ok(examples)
        })?.filter_map(Result::ok).collect();
        debug!("EXAMPLES: {examples:?}");

        debug!("Query: SELECT type FROM command_types WHERE COMMAND_ID={id};");
        let mut cmd_types_statment = conn.prepare(&format!("SELECT type FROM command_types WHERE command_id={id};"))?;
        let cmd_types: Vec<String> = cmd_types_statment.query_map([], |row| {
            let cmd_types = row.get(0)?;
            Ok(cmd_types)
        })?.filter_map(Result::ok).collect();
        let cmd_types = match cmd_types.get(0) {
            Some(s) => s.clone(),
            None => String::new()
        };
        debug!("TYPES: {cmd_types}");

        Ok(Command::new(
            id,
            name,
            name_exe,
            cmd_types,
            short_desc,
            details,
            args,
            examples))
    })?.filter_map(Result::ok).collect();

    info!("Number of loaded commands: {}", commands.len());

    return Ok(commands)
}