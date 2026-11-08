//! Commands started by a desktop app must not allocate their own Windows console.
use std::ffi::OsStr;

pub(crate) fn std_command(program: impl AsRef<OsStr>) -> std::process::Command {
    #[allow(unused_mut)]
    let mut command = std::process::Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    command
}

pub(crate) fn command(program: impl AsRef<OsStr>) -> tokio::process::Command {
    std_command(program).into()
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    // Report a real Win32 console handle, stdout/stderr, and a failing exit code.
    const PROBE: &str = r#"Add-Type -TypeDefinition 'using System; using System.Runtime.InteropServices; public class ConsoleProbe { [DllImport("kernel32.dll")] public static extern IntPtr GetConsoleWindow(); }'; [Console]::Out.WriteLine([ConsoleProbe]::GetConsoleWindow().ToInt64()); [Console]::Error.WriteLine('hidden diagnostic'); exit 7"#;

    fn assert_probe(output: std::process::Output) {
        assert_eq!(output.status.code(), Some(7));
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "0");
        assert!(String::from_utf8_lossy(&output.stderr).contains("hidden diagnostic"));
    }

    #[test]
    fn synchronous_command_has_no_console_and_preserves_failure_output() {
        assert_probe(
            std_command("powershell.exe")
                .args(["-NoProfile", "-NonInteractive", "-Command", PROBE])
                .output()
                .expect("PowerShell probe starts"),
        );
    }

    #[tokio::test]
    async fn asynchronous_command_has_no_console_and_preserves_failure_output() {
        assert_probe(
            command("powershell.exe")
                .args(["-NoProfile", "-NonInteractive", "-Command", PROBE])
                .output()
                .await
                .expect("PowerShell probe starts"),
        );
    }
}
