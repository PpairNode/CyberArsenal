# CyberArsenal INSTALL

> :warning: This has been tested on `ParrotOS 6.0 (lorikeet) - Base distro Debian`

Command to see distribution: `lsb_release -a`

## Install pre-requisites
```bash
sudo apt install xorg-dev libxcb-composite0-dev
```

## Run cargo
```bash
cargo run -- -s src/db.toml
```
