#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[allow(unused_variables)]
pub async fn get_browser_url(app_name: &str) -> Option<String> {
    #[cfg(target_os = "macos")]
    return macos::get_browser_url(app_name).await;

    #[cfg(target_os = "windows")]
    return windows::get_browser_url(app_name).await;

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return None;
}

pub fn extract_domain(url: &str) -> Option<String> {
    let without_protocol = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;

    let hostname = without_protocol.split(&['/', '?', '#', ':'][..]).next()?;

    if hostname.contains('.') {
        Some(hostname.to_lowercase())
    } else {
        None
    }
}

pub fn is_supported_browser(app_name: &str) -> bool {
    let name_lower = app_name.to_lowercase();
    matches!(
        name_lower.as_str(),
        "google chrome"
            | "chrome"
            | "safari"
            | "firefox"
            | "mozilla firefox"
            | "arc"
            | "microsoft edge"
            | "edge"
            | "brave"
            | "brave browser"
            | "opera"
            | "vivaldi"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain_https() {
        assert_eq!(
            extract_domain("https://mail.google.com/mail/u/0/#inbox"),
            Some("mail.google.com".to_string())
        );
    }

    #[test]
    fn test_extract_domain_http() {
        assert_eq!(
            extract_domain("http://example.com/path"),
            Some("example.com".to_string())
        );
    }

    #[test]
    fn test_extract_domain_with_port() {
        assert_eq!(
            extract_domain("https://example.com:8080/api"),
            Some("example.com".to_string())
        );
    }

    #[test]
    fn test_extract_domain_localhost_returns_none() {
        assert_eq!(extract_domain("https://localhost:3000/api"), None);
    }

    #[test]
    fn test_extract_domain_with_query() {
        assert_eq!(
            extract_domain("https://web.whatsapp.com/?tab=chat"),
            Some("web.whatsapp.com".to_string())
        );
    }

    #[test]
    fn test_extract_domain_uppercase() {
        assert_eq!(
            extract_domain("https://MAIL.GOOGLE.COM/"),
            Some("mail.google.com".to_string())
        );
    }

    #[test]
    fn test_extract_domain_no_protocol() {
        assert_eq!(extract_domain("mail.google.com"), None);
    }

    #[test]
    fn test_extract_domain_no_dot() {
        assert_eq!(extract_domain("https://localhost/"), None);
    }

    #[test]
    fn test_is_supported_browser() {
        assert!(is_supported_browser("Google Chrome"));
        assert!(is_supported_browser("Safari"));
        assert!(is_supported_browser("Firefox"));
        assert!(is_supported_browser("Arc"));
        assert!(is_supported_browser("Microsoft Edge"));
        assert!(is_supported_browser("Brave Browser"));
        assert!(!is_supported_browser("Visual Studio Code"));
        assert!(!is_supported_browser("Slack"));
    }
}
