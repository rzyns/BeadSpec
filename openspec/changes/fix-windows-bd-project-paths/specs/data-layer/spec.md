## ADDED Requirements

### Requirement: Windows-compatible Beads subprocess paths
BeadSpec SHALL pass conventional disk or UNC project paths to the Beads CLI
working directory and directory argument when its canonical project identity
uses a Windows verbatim disk or UNC prefix. Internal canonical identities SHALL
remain unchanged. Non-Windows paths and Windows device paths SHALL be preserved.

#### Scenario: Embedded project uses a canonical Windows disk path
- **WHEN** a project canonicalizes to `\\?\C:\repo`
- **THEN** Beads commands use `C:\repo` for the working directory and directory argument
- **AND** embedded issue reads succeed without a `//` database error

#### Scenario: Canonical UNC path is passed to Beads
- **WHEN** a project canonicalizes to `\\?\UNC\server\share\repo`
- **THEN** Beads commands use `\\server\share\repo`
- **AND** internal project and watcher identities retain the canonical path
