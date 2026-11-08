## ADDED Requirements

### Requirement: Background commands do not create Windows consoles
BeadSpec SHALL launch background commands without allocating Windows console
windows. It SHALL continue capturing stdout, stderr, and exit status, and SHALL
preserve existing timeouts and cancellation behavior.

#### Scenario: Periodic Git or Beads check succeeds
- **WHEN** a background check runs on Windows
- **THEN** no console window is allocated
- **AND** its successful output remains available to its caller

#### Scenario: A hidden command fails
- **WHEN** a background command exits unsuccessfully
- **THEN** its failure status and diagnostic output remain available
- **AND** the failure is not replaced with an empty successful result

#### Scenario: GitNexus refresh fails or returns no timestamp
- **WHEN** its CLI fails or does not provide a valid index timestamp
- **THEN** the status reports a diagnostic through the existing unavailable state
- **AND** it does not describe the index as fresh
- **AND** background queries do not download or prompt to install the optional CLI

#### Scenario: Beads reads and writes run without UI
- **WHEN** a Windows project is initialized and an issue is created and read
- **THEN** the real Beads commands complete successfully without console windows
- **AND** canonical Windows project identities continue to work
