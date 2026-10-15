# Writeup Platform AGENTS Instructions

## Overview

A self-hosted CTF writeup platform integrated with ret2shell. See `README.md` for configuration and setup instructions.

## Repository Structure

- **Backend**: `crates/server` (Axum service), `crates/database` (SQLx queries and migrations), and `crates/config` (configuration loading).
- **Frontend**: `frontend/`, using Vue 3 + TypeScript + Vite + Pinia; pages in `src/views`, components in `src/components`, API modules in `src/api`, and stores in `src/stores`.
- **Configuration example**: `config/config.toml.example`.

## Common Commands

- Start backend: `CONFIG_PATH=./config/config.toml SQLX_OFFLINE=true cargo run -p server`
- Check backend: `SQLX_OFFLINE=true cargo check --workspace`
- Format Rust: `cargo fmt --all`
- Test backend: `SQLX_OFFLINE=true cargo test --workspace`
- Install frontend dependencies: `npm --prefix frontend ci`
- Start frontend: `npm --prefix frontend run dev -- --host 0.0.0.0`
- Build frontend (includes type checking): `npm --prefix frontend run build`

## Development Convention

- Follow the existing code style, limit changes to the task, and avoid unrelated refactoring and new dependencies.

## Git Commits

- Use gitmoji shortcodes: `<gitmoji> <summary>`, e.g. `:memo: update development instructions`. Common choices: `:sparkles:` features, `:bug:` fixes, `:memo:` documentation, and `:recycle:` refactoring.
