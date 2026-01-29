//! Contract tests for CLI input/output

use std::io::Write;
use std::process::{Command, Stdio};

/// Test that the binary accepts valid stdin JSON
#[test]
fn test_binary_accepts_stdin_json() {
    let binary = env!("CARGO_BIN_EXE_claude-status");

    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn binary");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"{\"cwd\": \"/tmp\"}")
        .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read output");

    // Should exit successfully (exit code 0)
    assert!(
        output.status.success(),
        "Binary should exit successfully with valid JSON input"
    );

    // Should produce some output on stdout
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty(), "Should produce output on stdout");
}

/// Test that the binary outputs valid status line format
#[test]
fn test_binary_outputs_valid_status_line() {
    let binary = env!("CARGO_BIN_EXE_claude-status");

    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn binary");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let input = r#"{"cwd": "/Users/test/my-project", "model": {"display_name": "Sonnet 4"}}"#;
    stdin
        .write_all(input.as_bytes())
        .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read output");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify status line format components
    assert!(
        stdout.contains("📁"),
        "Should contain project name indicator"
    );
    assert!(
        stdout.contains("my-project"),
        "Should contain project name from cwd"
    );
    assert!(stdout.contains("🤖"), "Should contain model indicator");
    assert!(stdout.contains("S4"), "Should contain model short name");
    assert!(stdout.contains(" | "), "Parts should be pipe-separated");
}

/// Test that the binary handles empty JSON object
#[test]
fn test_binary_handles_empty_json() {
    let binary = env!("CARGO_BIN_EXE_claude-status");

    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn binary");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"{}").expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read output");

    // Should exit successfully even with empty JSON
    assert!(
        output.status.success(),
        "Binary should handle empty JSON gracefully"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("📁"),
        "Should still produce status line with defaults"
    );
}

/// Test that the binary reports errors to stderr
#[test]
fn test_binary_reports_errors_to_stderr() {
    let binary = env!("CARGO_BIN_EXE_claude-status");

    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn binary");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"{}").expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read output");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should report API error to stderr (since we won't have valid credentials in test)
    assert!(
        stderr.contains("API error") || stderr.contains("Keychain"),
        "Should report API/credential errors to stderr"
    );
}

/// Test that the binary handles invalid JSON
#[test]
fn test_binary_handles_invalid_json() {
    let binary = env!("CARGO_BIN_EXE_claude-status");

    let mut child = Command::new(binary)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn binary");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"not valid json")
        .expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to read output");

    // Should exit with error code for invalid input
    assert!(
        !output.status.success(),
        "Binary should fail with invalid JSON"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Failed to parse"),
        "Should report parse error to stderr"
    );
}

/// Test --version flag
#[test]
fn test_binary_version_flag() {
    let binary = env!("CARGO_BIN_EXE_claude-status");

    let output = Command::new(binary)
        .arg("--version")
        .output()
        .expect("Failed to run binary");

    assert!(output.status.success(), "--version should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("claude-status"),
        "Should show binary name in version"
    );
    assert!(stdout.contains("0.1.0"), "Should show version number");
}

/// Test --help flag
#[test]
fn test_binary_help_flag() {
    let binary = env!("CARGO_BIN_EXE_claude-status");

    let output = Command::new(binary)
        .arg("--help")
        .output()
        .expect("Failed to run binary");

    assert!(output.status.success(), "--help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage"), "Should show usage information");
    assert!(
        stdout.contains("--configure"),
        "Should list --configure option"
    );
}
