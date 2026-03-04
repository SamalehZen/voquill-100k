use std::time::Duration;
use tokio::task::spawn_blocking;
use tokio::time::timeout;
use windows::core::BSTR;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationElement, TreeScope_Descendants,
    UIA_ControlTypePropertyId, UIA_EditControlTypeId, UIA_ValueValuePropertyId,
};

const AUTOMATION_TIMEOUT: Duration = Duration::from_millis(500);

pub async fn get_browser_url(app_name: &str) -> Option<String> {
    let app_name = app_name.to_string();

    let result = timeout(
        AUTOMATION_TIMEOUT,
        spawn_blocking(move || get_browser_url_sync(&app_name)),
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

fn get_browser_url_sync(app_name: &str) -> Result<Option<String>, ()> {
    let name_lower = app_name.to_lowercase();
    let class_name = get_address_bar_class_name(&name_lower)?;

    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok();

        let result = (|| -> Option<String> {
            let automation: IUIAutomation =
                CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER).ok()?;

            let focused = automation.GetFocusedElement().ok()?;
            let top_level_window = walk_to_top_level_window(&automation, &focused)?;
            let url = find_address_bar_url(&automation, &top_level_window, class_name)?;

            Some(url)
        })();

        CoUninitialize();
        Ok(result)
    }
}

fn get_address_bar_class_name(app_name: &str) -> Result<&'static str, ()> {
    match app_name {
        "google chrome" | "chrome" | "microsoft edge" | "edge" | "brave" | "brave browser"
        | "opera" | "vivaldi" => Ok("OmniboxViewViews"),
        "firefox" | "mozilla firefox" => Ok("MozillaCompositorWindowClass"),
        _ => Err(()),
    }
}

unsafe fn walk_to_top_level_window(
    automation: &IUIAutomation,
    element: &IUIAutomationElement,
) -> Option<IUIAutomationElement> {
    let walker = automation.CreateTreeWalker(&automation.ContentViewCondition().ok()?).ok()?;

    let mut current = element.clone();
    let mut top_level: Option<IUIAutomationElement> = None;

    loop {
        let parent = walker.GetParentElement(&current).ok()?;
        let parent_parent = walker.GetParentElement(&parent).ok();

        if parent_parent.is_none() || parent_parent.as_ref().map(|p| p.CurrentClassName().ok()).flatten().is_none() {
            top_level = Some(current);
            break;
        }

        current = parent;
    }

    top_level
}

unsafe fn find_address_bar_url(
    automation: &IUIAutomation,
    window: &IUIAutomationElement,
    class_name: &str,
) -> Option<String> {
    let edit_condition = automation
        .CreatePropertyCondition(
            UIA_ControlTypePropertyId,
            &VARIANT::from(UIA_EditControlTypeId.0 as i32),
        )
        .ok()?;

    let elements = window
        .FindAll(TreeScope_Descendants, &edit_condition)
        .ok()?;

    let count = elements.Length().ok()?;
    for i in 0..count {
        if let Ok(element) = elements.GetElement(i) {
            if let Ok(element_class) = element.CurrentClassName() {
                let class_str = element_class.to_string();
                if class_str.contains(class_name) {
                    if let Some(url) = get_element_value(&element) {
                        if url.starts_with("http://") || url.starts_with("https://") {
                            return Some(url);
                        }
                        if url.contains('.') && !url.contains(' ') {
                            return Some(format!("https://{}", url));
                        }
                    }
                }
            }
        }
    }

    None
}

unsafe fn get_element_value(element: &IUIAutomationElement) -> Option<String> {
    let value: VARIANT = element
        .GetCurrentPropertyValue(UIA_ValueValuePropertyId)
        .ok()?;

    let vt = value.Anonymous.Anonymous.vt;
    if vt != windows::Win32::System::Variant::VT_BSTR.0 {
        return None;
    }

    let bstr: BSTR = std::mem::transmute_copy(&value.Anonymous.Anonymous.Anonymous.bstrVal);
    if bstr.is_empty() {
        return None;
    }

    Some(bstr.to_string())
}
