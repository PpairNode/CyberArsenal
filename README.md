# CyberArsenal README
CyberArsenal is an inventory, reminder and launcher for linux/windows commands of all types (forensics, pentest, development, sysadmin, ...).

## Building DB
To build the database, follow the instruction here: [Database installation](setup/README.md).

## Running CyberArsenal
To run the tool, for now it is only available through cargo locally: `cargo run -- -s <file.db>` and replace `file.db` with your created during the setup.

Here is a quick demo about the tool:

![CyberArsenal](docs/cyberarsenal.gif)

## CyberArsenal
- Using python and SQLite to create a database from a commands file toml
- Search bar
- List of commands to chose
- Hit `ENTER/EXIT`: enter or exit the command popup
- Inside command popup, copy/paste with `ENTER`. It copies the content of the command and its modified values.
- Information pane
- Command are loaded from a `SQLite` file created by the setup installation procedures.

## Authors
- PpairNode

## TODO
For more information about what is next, see [TODO](docs/TODO.md).
