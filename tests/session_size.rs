//! Session size integration tests

use claude_status::display::status_line::StatusLineBuilder;
use claude_status::domain::input::Model;
use claude_status::domain::session::SessionSize;

#[test]
fn test_valid_session_size_displayed() {
    let size = SessionSize::new(2 * 1024 * 1024); // 2MB

    let line = StatusLineBuilder::new()
        .project_name("test")
        .model(Model::from_display_name("Sonnet 4"))
        .session_size(size)
        .build();

    assert!(line.contains("📄"), "Should contain session size indicator");
    assert!(line.contains("2.0MB"), "Should contain formatted size");
}

#[test]
fn test_session_size_omitted_when_not_provided() {
    let line = StatusLineBuilder::new()
        .project_name("test")
        .model(Model::from_display_name("Sonnet 4"))
        .build();

    assert!(
        !line.contains("📄"),
        "Should not contain session size indicator"
    );
}

#[test]
fn test_session_size_warning_threshold() {
    let size = SessionSize::new(8 * 1024 * 1024); // 8MB - warning

    let line = StatusLineBuilder::new()
        .project_name("test")
        .session_size(size)
        .build();

    assert!(line.contains("📄"), "Should contain session size indicator");
    assert!(line.contains("⚠️"), "Should contain warning indicator");
}

#[test]
fn test_session_size_critical_threshold() {
    let size = SessionSize::new(20 * 1024 * 1024); // 20MB - critical

    let line = StatusLineBuilder::new()
        .project_name("test")
        .session_size(size)
        .build();

    assert!(line.contains("📄"), "Should contain session size indicator");
    assert!(line.contains("🔴"), "Should contain critical indicator");
}

#[test]
fn test_session_size_from_file() {
    use std::io::Write;

    // Create a temp file with known size
    let mut temp = tempfile::NamedTempFile::new().unwrap();
    let data = vec![0u8; 512 * 1024]; // 512KB
    temp.write_all(&data).unwrap();
    temp.flush().unwrap();

    let size = SessionSize::from_file(temp.path()).unwrap();

    let line = StatusLineBuilder::new()
        .project_name("test")
        .session_size(size)
        .build();

    assert!(line.contains("512KB"), "Should contain file size");
}
