# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

## [v.0.0.3] - 2026-02-18
### Added
- Search: shows the current command selected
- Search: shows the number of filtered commands
- Popup: command arguments with same name -> shown once in `Arguments` only and all modified at the same time
- Popup: add `BASE` and `UPDATED` info to keep track of modification depending on base command
- DB value: `local` -> use for local or remote purpose
- DB value: `use_name` -> name that appears on the command panel (used when command name is not clear)
- Color code of the 3 central body columns
- New commands as example

### Fixed
- Pre/Post arg on commands

### Changed
- UI and colors => more readable
- Docs

### Removed
- DB values: `name_exe`

## [v0.0.2] - 2025-06-22
### Added
- SQLite database
- Database builder from TOML commands file

### Fixed
- Popup + Info pane

### Changed
- Debug logs to external file

### Removed
- Log pane

---

## [v0.0.1] - 2024-11-08
### Added
- TUI
- Show command
- Info pane
- Popup pane
- Search bar
- Command parser
- TOML commands file
