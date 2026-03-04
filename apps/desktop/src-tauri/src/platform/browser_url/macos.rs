use std::process::Command;
use std::time::Duration;
use tokio::task::spawn_blocking;
use tokio::time::timeout;

const APPLESCRIPT_TIMEOUT: Duration = Duration::from_millis(500);

pub async fn get_browser_url(app_name: &str) -> Option<String> {
    let script = get_applescript_for_browser(app_name)?;

    let result = timeout(
        APPLESCRIPT_TIMEOUT,
        spawn_blocking(move || execute_applescript(&script)),
    )
    .await;

    match result {
        Ok(Ok(url)) => url,
        Ok(Err(_)) => None,
        Err(_) => {
            tracing::debug!("Browser URL detection timed out for {}", app_name);
            None
        }
    }
}

fn execute_applescript(script: &str) -> Option<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .ok()?;

    if output.status.success() {
        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !url.is_empty() && url != "missing value" {
            return Some(url);
        }
    }
    None
}

fn get_applescript_for_browser(app_name: &str) -> Option<String> {
    let name_lower = app_name.to_lowercase();

    let script = match name_lower.as_str() {
        "google chrome" | "chrome" => {
            r#"tell application "Google Chrome" to get URL of active tab of front window"#
        }
        "safari" => {
            r#"tell application "Safari" to get URL of current tab of front window"#
        }
        "firefox" | "mozilla firefox" => {
            r#"tell application "System Events"
                tell process "Firefox"
                    set frontWindow to front window
                    set urlBar to text field 1 of toolbar 1 of frontWindow
                    return value of urlBar
                end tell
            end tell"#
        }
        "arc" => r#"tell application "Arc" to get URL of active tab of front window"#,
        "microsoft edge" | "edge" => {
            r#"tell application "Microsoft Edge" to get URL of active tab of front window"#
        }
        "brave" | "brave browser" => {
            r#"tell application "Brave Browser" to get URL of active tab of front window"#
        }
        "opera" => r#"tell application "Opera" to get URL of active tab of front window"#,
        "vivaldi" => r#"tell application "Vivaldi" to get URL of active tab of front window"#,
        _ => return None,
    };

    Some(script.to_string())
}
