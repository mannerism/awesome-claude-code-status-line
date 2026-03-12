//! Claude Code status line generator
//!
//! Reads JSON input from stdin and outputs a formatted status line.

use std::io::{self, Read};

use clap::Parser;

use claude_status::api::{client::fetch_usage, keychain::get_access_token};
use claude_status::display::status_line::StatusLineBuilder;
use claude_status::domain::context::ContextUsageInfo;
use claude_status::domain::input::ClaudeInput;
use claude_status::StatusLineError;

/// Claude Code status line generator
#[derive(Parser, Debug)]
#[command(name = "claude-status")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run interactive configuration
    #[arg(long)]
    configure: bool,
}

fn main() {
    let args = Args::parse();

    if args.configure {
        // TODO: Implement interactive configuration
        eprintln!("Configuration mode not yet implemented.");
        std::process::exit(0);
    }

    // Read JSON from stdin
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read stdin: {}", e);
        std::process::exit(1);
    }

    // Parse input
    let claude_input: ClaudeInput = match serde_json::from_str(&input) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Failed to parse input JSON: {}", e);
            std::process::exit(1);
        }
    };

    // Build status line
    let status_line = build_status_line(&claude_input);
    print!("{}", status_line);
}

/// Build the complete status line from input
fn build_status_line(input: &ClaudeInput) -> String {
    let mut builder = StatusLineBuilder::new();

    // Model
    let model = input.get_model();
    builder = builder.model(model);

    // Context window usage
    if let Some(ref ctx) = input.context_window {
        if let (Some(ref usage), Some(total)) = (&ctx.current_usage, ctx.context_window_size) {
            let used = usage.input_tokens
                + usage.cache_creation_input_tokens
                + usage.cache_read_input_tokens;
            let ctx_usage = ContextUsageInfo::new(used, total);
            builder = builder.context_usage(ctx_usage);
        }
    }

    // Usage data from API
    match get_usage_data() {
        Ok((five_hour, seven_day)) => {
            builder = builder.five_hour(five_hour).seven_day(seven_day);
        }
        Err(e) => {
            // Show brief error in status line
            if e.show_in_status_line() {
                builder = builder.error(e.brief());
            }
            // Full error to stderr
            eprintln!("API error: {}", e);
        }
    }

    builder.build()
}

/// Get usage data from Anthropic API
fn get_usage_data() -> Result<
    (
        claude_status::domain::usage::CycleInfo,
        claude_status::domain::usage::CycleInfo,
    ),
    StatusLineError,
> {
    let token = get_access_token()?;
    let response = fetch_usage(&token)?;
    response.to_domain()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_status_line_empty_input() {
        let input = ClaudeInput::default();
        let line = build_status_line(&input);
        // Should have at least a model
        assert!(line.contains("🤖"));
    }
}
