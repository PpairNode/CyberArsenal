# CyberArsenal README

CyberArsenal is an inventory, reminder and launcher for linux/windows commands of all types (forensics, pentest, development, sysadmin, ...).

## Demonstration
To run the tool, for now it is only available through cargo locally: `cargo run -- -s src/db.toml`

Here is a quick demo about the tool:

![CyberArsenal](docs/cyberarsenal.gif)

## CyberArsenal
- Search bar
- List of commands to chose
- Hit `ENTER/EXIT`: enter or exit the command popup
- Inside command popup, hit `ENTER`: copy the content of the command and its modified values
- Information panel
- Command are loaded from a `.toml` file

### Structure of settings file
```t
title = "Pentest commands"
max_events = 100

[command.objdump0]
name_exe = "objdump"
cmd_type = "reverse"
short_desc = "Display information from object <binary> in <type> format"
details = "..."
args = "-D <binary> -M <type=intel>"
examples = [
    "obdjump -M intel -D a.out"
]
```

A `[command.xxx]` block will load this as a command.
All command keys are optional inside a command block.
Understandable keys are:
- `name_exe`: name of the command
- `cmd_types`: A `|` separated list of types for the command (supported are `pentest|forensics|programming|reverse|crypto|network|sysadmin`)
- `short_desc`: an short description of the command
- `details`: details if the tool need more info
- `args`: all arguments of this command
    - An arg between `<k>`, will have a `k` key which can be modified from the tool.
    - An arg prompted like this `<k|v>`, has a auto-filled `k` key with the `v` value by default.
- `examples`: array of examples of how to use the command

## Authors
- PpairNode

## TODO
For more information about what is next, see [TODO](docs/TODO.md).
