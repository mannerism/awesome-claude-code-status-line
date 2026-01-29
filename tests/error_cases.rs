//! Error handling integration tests

use claude_status::display::status_line::StatusLineBuilder;
use claude_status::domain::input::Model;
use claude_status::error::StatusLineError;

#[test]
fn test_status_line_with_error() {
    let line = StatusLineBuilder::new()
        .project_name("test")
        .model(Model::from_display_name("Sonnet 4"))
        .error("No creds")
        .build();

    assert!(line.contains("⚠️ No creds"), "Should show error indicator");
    // Should not contain usage data when there's an error
    assert!(
        !line.contains("⚡"),
        "Should not show 5-hour cycle when error"
    );
    assert!(
        !line.contains("📅"),
        "Should not show 7-day cycle when error"
    );
}

#[test]
fn test_keychain_error_brief_message() {
    let error =
        StatusLineError::KeychainAccess("The specified item could not be found".to_string());
    assert_eq!(error.brief(), "No creds");
    assert!(error.show_in_status_line());
}

#[test]
fn test_credentials_not_found_error_brief_message() {
    let error = StatusLineError::CredentialsNotFound;
    assert_eq!(error.brief(), "No creds");
    assert!(error.show_in_status_line());
}

#[test]
fn test_api_request_error_brief_message() {
    let error = StatusLineError::ApiRequest("Connection timeout".to_string());
    assert_eq!(error.brief(), "API error");
    assert!(error.show_in_status_line());
}

#[test]
fn test_api_response_error_brief_message() {
    let error = StatusLineError::ApiResponse("Invalid JSON".to_string());
    assert_eq!(error.brief(), "API error");
    assert!(error.show_in_status_line());
}

#[test]
fn test_git_error_not_shown_in_status_line() {
    let error = StatusLineError::Git("Not a git repository".to_string());
    assert!(
        !error.show_in_status_line(),
        "Git errors should not show in status line"
    );
}

#[test]
fn test_error_display_full_message() {
    let error = StatusLineError::KeychainAccess("Detailed error message".to_string());
    let display = format!("{}", error);
    assert!(
        display.contains("Detailed error message"),
        "Display should contain full details"
    );
}
