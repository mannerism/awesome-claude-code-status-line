#!/usr/bin/env python3
"""
Status line generator for Claude Code integration.
Reads JSON input from stdin and outputs formatted status line.
"""

import json
import sys
import time
import os
import subprocess
import urllib.request
import urllib.error
from pathlib import Path
from datetime import datetime
from tracker import UsageTracker
from config import Config
from git_info import GitInfo


def get_api_usage():
    """Get actual usage from Anthropic API."""
    try:
        # Get credentials from macOS Keychain
        result = subprocess.run(
            ["security", "find-generic-password", "-s", "Claude Code-credentials", "-w"],
            capture_output=True, text=True, timeout=5
        )
        if result.returncode != 0:
            return None

        creds = json.loads(result.stdout.strip())
        token = creds.get("claudeAiOauth", {}).get("accessToken")
        if not token:
            return None

        # Call the usage API
        url = "https://api.anthropic.com/api/oauth/usage"
        req = urllib.request.Request(url, headers={
            "Authorization": f"Bearer {token}",
            "anthropic-beta": "oauth-2025-04-20",
            "User-Agent": "claude-code/2.0.31",
            "Content-Type": "application/json"
        })

        with urllib.request.urlopen(req, timeout=5) as response:
            return json.loads(response.read().decode())
    except Exception:
        return None


def format_reset_time_kst(resets_at: str) -> str:
    """Format reset time as KST date and time."""
    try:
        from datetime import timezone, timedelta
        reset_dt = datetime.fromisoformat(resets_at.replace('Z', '+00:00'))
        # Convert to KST (UTC+9)
        kst = timezone(timedelta(hours=9))
        reset_kst = reset_dt.astimezone(kst)
        return reset_kst.strftime("%m/%d %H:%M")
    except Exception:
        return "?"

def generate_status_line():
    """Generate status line output for Claude Code."""

    # Read JSON from stdin (Claude Code provides this)
    stdin_data = None
    try:
        stdin_data = json.load(sys.stdin)
    except Exception:
        pass

    # Get current project name from stdin data (Claude Code provides cwd)
    try:
        if stdin_data and stdin_data.get('cwd'):
            project_path = stdin_data['cwd']
        else:
            project_path = os.getcwd()
        project_name = Path(project_path).name
    except:
        project_name = 'unknown'

    # Initialize components
    config = Config()
    git_info = GitInfo(cache_duration=config.git_cache_duration)

    # Try to get actual API usage first
    api_usage = get_api_usage()

    # Format parts
    parts = []

    # Project name
    parts.append(f"📁 {project_name}")

    # Add git information if enabled
    if config.show_git_info:
        git_status = git_info.get_git_status(project_path)
        git_display = git_info.format_git_info(git_status)
        if git_display:
            parts.append(git_display)

    # Current model from stdin data or fallback detection
    current_model = "Sonnet 4"  # Default

    if stdin_data and stdin_data.get('model'):
        current_model = stdin_data['model'].get('display_name', 'Sonnet 4')
    else:
        # Fallback: Check environment variable
        claude_model = os.environ.get('CLAUDE_MODEL', '').lower()
        if 'opus' in claude_model:
            current_model = "Opus 4.5"
        elif 'haiku' in claude_model:
            current_model = "Haiku 4.5"
        elif 'sonnet' in claude_model:
            current_model = "Sonnet 4.5"
        else:
            # Read from settings.json
            try:
                settings_path = Path.home() / ".claude" / "settings.json"
                if settings_path.exists():
                    with open(settings_path, 'r') as f:
                        settings = json.load(f)
                        model_setting = settings.get('model', '').lower()
                        if 'opus' in model_setting:
                            current_model = "Opus 4.5"
                        elif 'haiku' in model_setting:
                            current_model = "Haiku 4.5"
                        elif 'sonnet' in model_setting:
                            current_model = "Sonnet 4.5"
            except Exception:
                pass

    parts.append(f"🤖 {current_model}")

    # Context window usage from stdin data
    if stdin_data and stdin_data.get('context_window'):
        ctx = stdin_data['context_window']
        usage = ctx.get('current_usage')
        window_size = ctx.get('context_window_size', 200000)

        if usage:
            total_tokens = (
                usage.get('input_tokens', 0) +
                usage.get('cache_creation_input_tokens', 0) +
                usage.get('cache_read_input_tokens', 0)
            )
            ctx_pct = (total_tokens / window_size) * 100 if window_size > 0 else 0

            # Color based on context usage
            if ctx_pct >= 80:
                ctx_color = (255, 100, 100)  # Red
            elif ctx_pct >= 50:
                ctx_color = (255, 255, 0)    # Yellow
            else:
                ctx_color = (100, 200, 255)  # Blue

            parts.append(f"\033[38;2;{ctx_color[0]};{ctx_color[1]};{ctx_color[2]}m📊 {ctx_pct:.0f}%\033[0m")

    if api_usage:
        # Use actual API data
        five_hour = api_usage.get("five_hour", {})
        seven_day = api_usage.get("seven_day", {})

        # 5-hour session usage
        session_pct = five_hour.get("utilization", 0)
        session_reset = format_reset_time_kst(five_hour.get("resets_at", ""))

        # Color based on percentage
        if session_pct >= 80:
            color = (255, 100, 100)  # Red
        elif session_pct >= 50:
            color = (255, 255, 0)    # Yellow
        else:
            color = (0, 255, 0)      # Green

        parts.append(f"\033[38;2;{color[0]};{color[1]};{color[2]}m⚡ {session_pct:.0f}% @{session_reset}\033[0m")

        # Weekly usage
        weekly_pct = seven_day.get("utilization", 0)
        weekly_reset = format_reset_time_kst(seven_day.get("resets_at", ""))

        if weekly_pct >= 80:
            wcolor = (255, 100, 100)
        elif weekly_pct >= 50:
            wcolor = (255, 255, 0)
        else:
            wcolor = (0, 255, 0)

        parts.append(f"\033[38;2;{wcolor[0]};{wcolor[1]};{wcolor[2]}m📅 {weekly_pct:.0f}% @{weekly_reset}\033[0m")
    else:
        # Fallback to local calculation
        from datetime import timezone, timedelta
        tracker = UsageTracker()
        usage = tracker.update()
        limits = config.get_tier_limits()

        now = time.time()
        cycle_end = usage.current_5h_start + (5 * 3600)

        # Convert cycle_end to KST
        kst = timezone(timedelta(hours=9))
        reset_dt = datetime.fromtimestamp(cycle_end, tz=kst)
        reset_kst = reset_dt.strftime("%m/%d %H:%M")

        percentage = int((usage.current_5h_prompts / limits.cycle_5h_max) * 100) if limits.cycle_5h_max > 0 else 0
        color = config.get_usage_color(usage.current_5h_prompts, limits.cycle_5h_max)
        parts.append(f"\033[38;2;{color[0]};{color[1]};{color[2]}m⚡ {percentage}% @{reset_kst} (est)\033[0m")

    # Output the status line
    status_line = " | ".join(parts)
    print(status_line)

if __name__ == "__main__":
    generate_status_line()