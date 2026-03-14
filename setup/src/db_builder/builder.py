import argparse
import tomllib
from pathlib import Path
import logging
import sqlite3
from sqlite3 import Connection

from db_builder.logs import init_logs


# ========== TABLES ==========
# Commands:
# - ID (INTEGER): ID of command itself
# - NAME_EXE (TEXT): command name
# - CMD_TYPES (TEXT): type? type1|type2|type3? handle multiple types or not?
# - SHORT_DESC (TEXT): a briefe description of the command
# - DETAILS (TEXT): more details if a command needs some
# - args (TEXT): arguments of the command

# Example:
# - ID (INTEGER): ID of the command it references
# - CMD_EXAMPLE (TEXT): example


# +----+-------+-----------+-------------------------------+
# | ID | NAME  | NAME_EXE  | SHORT_DESC                    |
# +----+-------+-----------+-------------------------------+
table_commands="""
-- Main table for commands
CREATE TABLE IF NOT EXISTS commands (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    local INTEGER DEFAULT 1,
    key TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    short_desc TEXT,
    details TEXT
);"""

# +------------+---------+
# | COMMAND_ID | TYPE    |
# +------------+---------+
table_command_types="""
-- Table for command types (many-to-one relation with commands)
CREATE TABLE IF NOT EXISTS command_types (
    command_id INTEGER,
    type TEXT,
    FOREIGN KEY (command_id) REFERENCES commands(id)
);"""

# +------------+----------+
# | COMMAND_ID | ARGS     |
# +------------+----------+
table_command_arguments="""
-- Table for arguments (many-to-one relation with commands)
CREATE TABLE IF NOT EXISTS command_args (
    command_id INTEGER,
    args TEXT,
    FOREIGN KEY (command_id) REFERENCES commands(id)
);"""

# +------------+--------------+
# | COMMAND_ID | EXAMPLES     |
# +------------+--------------+
table_command_examples="""
-- Table for examples (many-to-one relation with commands)
CREATE TABLE IF NOT EXISTS command_examples (
    command_id INTEGER,
    example TEXT,
    FOREIGN KEY (command_id) REFERENCES commands(id)
);"""

TABLES = [
    table_commands,
    table_command_types,
    table_command_arguments,
    table_command_examples
]


# INSERT DATA
# -- Insert command
# INSERT INTO commands (local, key, use_name, short_desc, details) VALUES (?,?,?,?,?);

# -- Add types
# INSERT INTO command_types (command_id, type) VALUES (?,?);

# -- Add arguments
# INSERT INTO command_args (command_id, args) VALUES (?,?);

# -- Add examples
# INSERT INTO command_examples (command_id, example) VALUES (?,?);


def create_tables(conn: Connection, tables: list[str]) -> bool:
    cursor = conn.cursor()
    for table in tables:
        cursor.execute(table)
    conn.commit()


def insert_data(conn: Connection, toml_data: dict[str, any]) -> bool:
    cursor = conn.cursor()
    for table_name, table_item in toml_data.items():
        # Parse all commands
        if table_name == "command":
            for key, val in table_item.items():
                # Get values
                use_name, local, cmd_types, short_desc, details, args, examples = [""] * 7
                if 'use_name' in val:
                    use_name = val['use_name']
                if 'local' in val:
                    local = 1 if val['local'] else 0
                if 'cmd_types' in val:
                    cmd_types = val['cmd_types']
                if 'short_desc' in val:
                    short_desc = val['short_desc']
                if 'details' in val:
                    details = val['details']
                if 'args' in val:
                    args = val['args']
                if 'examples' in val:
                    examples = val['examples']

                # Only if use_name if empty use key instead
                if use_name == "":
                    use_name = key
                # Now insert values for this command
                cursor.execute("INSERT OR IGNORE INTO commands (local, key, name, short_desc, details) VALUES (?,?,?,?,?);", (local, key, use_name, short_desc, details))
                id = cursor.lastrowid
                if id == 0:
                    logging.warning(f"Duplicate name skipped: {key}")
                    continue
                cursor.execute(f"INSERT INTO command_types (command_id, type) VALUES (?,?);",
                               (id, cmd_types))
                cursor.execute(f"INSERT INTO command_args (command_id, args) VALUES (?,?);",
                               (id, args))
                for example in examples:
                    cursor.execute(f"INSERT INTO command_examples (command_id, example) VALUES (?,?);",
                                   (id, example))
                logging.debug(f"Command: [{key}]{use_name} has been added")
    conn.commit()


def connect_db(name) -> Connection | None :
    try:
        return sqlite3.connect(name)
    except Exception as e:
        raise e

def confirm(path: Path) -> bool:
    return input(f"{path} already exists, delete it? [y/N] ").strip().lower() in ("y", "yes")


def main():
    parser = argparse.ArgumentParser(description="SQLite Builder for CyberArsenal")
    parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose output")
    parser.add_argument("-d", "--database-name", type=str, help="Name of SQLite DB file", default="sqlite.db")
    parser.add_argument("-c", "--config", required=True, type=str, help="Config file to add files into DB (toml)", default="config.toml")
    parser.add_argument("-o", "--overwrite-db", action="store_true", help="Overwrite DB (delete actual)")
    parser.add_argument("-f", "--force-backup-overwrite", action="store_true", help="Overwrite old DB when used with `-b`")
    parser.add_argument("-b", "--backup", action="store_true", help="Backup DB")

    args = parser.parse_args()

    init_logs(args.verbose)

    path = Path(args.database_name)

    # Check backup and if exists check force then delete
    if args.backup:
        path_bak = Path(args.database_name + ".bak")
        if path_bak.is_file():
            if not args.force_backup_overwrite:
                if confirm(path_bak):
                    path_bak.unlink()
                    logging.info(f"Old DB has been deleted: {path_bak}")
            else:
                path_bak.unlink(missing_ok=True)
                logging.info(f"Old DB has been deleted: {path_bak}")
            path.rename(path_bak)
            logging.info(f"DB has been backed to {path_bak}")

    # Check file and check force then delete
    if args.overwrite_db and path.is_file():
        path.unlink(missing_ok=True)
        logging.info(f"DB has been deleted: {path}!")
    else:
        logging.info(f"New data will be added to DB: {path}")

    # Open DB
    conn: Connection = connect_db(path)

    # Create DB tables
    create_tables(conn, TABLES)

    # Load TOML for DB data
    with open(args.config, "rb") as f:
        config_toml = tomllib.load(f)

    # Enter DB data
    for filepath in config_toml['files']:
        with open(filepath, "rb") as f:
            toml_data = tomllib.load(f)
        insert_data(conn, toml_data)

    # Close DB
    conn.close()

    logging.info(f"New DB located at: {path}")
    
