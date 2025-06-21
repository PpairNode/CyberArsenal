# Installation of DB

## Virtual ENV and installation of db_builder
```bash
python3 -m venv .env
source .env/bin/activate

pip3 install -e .
```

## Run DB builder
```bash
# This build the DB file
db_builder -f cmds.toml -v

# Open and check values has been inserted
sqlitebrowser sqlite.db
```

## Cleaning Installation
```bash
make clean
```
