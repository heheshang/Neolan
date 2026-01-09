# Change: Code Quality Cleanup and Optimization

## Why

The codebase has accumulated compiler warnings that indicate:
- Unused variables and imports that clutter the code
- Deprecated method usage that needs updating
- Dead code (unused structs and methods) that increases maintenance burden
- Incomplete TODOs in critical file transfer functionality

These issues reduce code maintainability and may mask real bugs.

## What Changes

- **Remove unused imports** in `state/events.rs`, `config/app.rs`, and `modules/file_transfer/mod.rs`
- **Prefix intentionally unused variables** with underscore (`_uuid`, `_local_ip`, `_peer_ip`)
- **Update deprecated chrono API calls** from `timestamp_millis()` to `.and_utc().timestamp_millis()`
- **Review and remove dead code** in `config/app.rs` (ConfigRepository and its methods are marked as never constructed)
- **Document or implement incomplete TODOs** in file transfer commands

## Impact

- Affected specs: None (internal code quality improvement)
- Affected code:
  - `src-tauri/src/commands/file_transfer.rs` (unused variables)
  - `src-tauri/src/commands/message.rs` (deprecated chrono, unused variables)
  - `src-tauri/src/state/events.rs` (unused imports)
  - `src-tauri/src/config/app.rs` (unused imports, dead code)
  - `src-tauri/src/modules/file_transfer/mod.rs` (unused imports)
- Risk: Low - changes are limited to removing dead code and fixing warnings
