# core-service

**Kind:** binary
**Depends on:** `service`

## Purpose
Thin entry point: loads `config/core.toml`, starts the `service` subsystems and runs until stopped. Intended to run as a system service.

## Rules
- No logic beyond startup, shutdown and signal handling. Everything else belongs in `service`.
- Creates the Unix socket directory (`/run/big-boss/`) with group permissions for the desktop.
