## Why

On Windows, filesystem canonicalization produces verbatim paths such as
`\\?\C:\repo`. Beads 1.3.2 can report version and context for those paths,
but embedded database reads fail with `open db: '//' does not exist`.

## What Changes

Convert canonical Windows disk and UNC project paths to conventional Windows
paths at the `bd` subprocess boundary. Apply the conversion to the working
directory and the `-C` argument for native commands and the HTTP server.
Keep canonical project identities, registry keys, and watcher paths intact.

## Capabilities

### Modified Capabilities
- `data-layer`: Windows project paths passed to Beads must be usable by its database backend.

## Impact

The Rust Beads command runners and project transport are affected. Tauri command
signatures and React components do not change. No dependencies are added.
Server-mode execution uses the same conversion, but the native acceptance test
covers embedded mode. Task tracking lives in Beads.
