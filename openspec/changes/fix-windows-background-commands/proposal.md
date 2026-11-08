## Why

Background Beads and Git commands currently create transient console windows on
Windows. The GitNexus badge refreshes every 60 seconds, while Beads transport and
health polling also launch console executables. Piping output alone does not
prevent Windows from allocating a console.

## What Changes

Use a shared background-command constructor for synchronous and asynchronous
subprocesses. Set CREATE_NO_WINDOW on Windows without changing command arguments,
environment, output pipes, exit statuses, timeouts, or managed server lifetimes.
Cover Beads reads/writes, HTTP server startup, generic tool commands, Dolt
recovery startup, and actor lookup.

Stop treating failed GitNexus status commands or missing index timestamps as a
fresh index. Preserve those diagnostics in the existing unavailable-status UI.
Background GitNexus queries use an installed/cached CLI without downloading or
prompting to install the optional tool.
Use the supported `status --json` command for the current project, including its
explicit not-indexed/error and freshness fields; `list` has no `--json` option in
the installed GitNexus 1.6.12 CLI.

Validate hidden execution with a native console probe and real Beads operations,
including failure output. No Tauri command signatures or dependencies change.

## Capabilities

### Modified Capabilities
- `data-layer`: background commands execute without console windows and preserve results.

## Impact

The Rust subprocess launch boundaries and Windows acceptance tests are affected.
Task tracking lives in BEADSPEC-83rxa. Package the verified fix as a new Scoop
revision; retain earlier release artifacts unchanged.
