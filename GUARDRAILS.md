# GUARDRAILS.md - Digital Product Factory (Desktop)

**Rust Guardrails - Vibe Engineering Standard**

## Context
Native Rust desktop app built with egui/eframe. Not a web server - different rules apply.

## Non-Negotiable
- GUI callbacks (lock().unwrap() on mutexes): acceptable in UI event loops where failure means the app is already in an unrecoverable state. But prefer lock().ok()? or error logging where possible.
- File I/O: ALL file operations must handle errors gracefully - log and surface to user, never silently crash.
- Database: rusqlite queries must never unwrap() results in production paths. Use ? or match.
- No expect() with hardcoded messages - if the app fails, it should fail informatively.
- cargo clippy -- -D warnings must pass before any task is declared done.
- Desktop build only - no deployment to server. Build locally, ship binary.

## Verification Before Build
1. cargo check
2. cargo clippy -- -D warnings
3. cargo test
4. cargo build --release (produces standalone binary)
