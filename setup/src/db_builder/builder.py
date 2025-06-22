import argparse
import tomllib
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
    name TEXT NOT NULL,
    name_exe TEXT NOT NULL,
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
# INSERT INTO commands (name, name_exe, short_desc, details) VALUES (?,?,?,?);

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
                name_exe, cmd_types, short_desc, details, args, examples = [""] * 6
                if 'name_exe' in val:
                    name_exe = val['name_exe']
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
                # Now insert values for this command
                cursor.execute(f"INSERT INTO commands (name, name_exe, short_desc, details) VALUES (?,?,?,?);",
                               (key, name_exe, short_desc, details))
                id = cursor.lastrowid
                cursor.execute(f"INSERT INTO command_types (command_id, type) VALUES (?,?);",
                               (id, cmd_types))
                cursor.execute(f"INSERT INTO command_args (command_id, args) VALUES (?,?);",
                               (id, args))
                for example in examples:
                    cursor.execute(f"INSERT INTO command_examples (command_id, example) VALUES (?,?);",
                                   (id, example))
    conn.commit()


def connect_db(name) -> Connection | None :
    try:
        return sqlite3.connect(name)
    except Exception as e:
        raise e


def main():
    parser = argparse.ArgumentParser(description="SQLite Builder for CyberArsenal")
    parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose output")
    parser.add_argument("-d", "--database-name", type=str, help="Name of SQLite DB file", default="sqlite.db")
    parser.add_argument("-f", "--file", required=True, type=str, help="File to add commands into DB (toml)", default="commands.toml")

    args = parser.parse_args()

    init_logs(args.verbose)

    # Open DB
    conn: Connection = connect_db(args.database_name)

    # Create DB tables
    create_tables(conn, TABLES)

    # Load TOML for DB data
    with open(args.file, "rb") as f:
        toml_data = tomllib.load(f)

    # Enter DB data
    insert_data(conn, toml_data)

    # Close DB
    conn.close()
    
