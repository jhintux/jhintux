//! Small helpers shared across the app.

/// Smooth-scroll to an element by id (web). No-op on other targets if eval is unavailable.
pub fn scroll_to_id(id: &str) {
    let id = id
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>();
    if id.is_empty() {
        return;
    }
    #[cfg(feature = "web")]
    {
        let _ = dioxus::prelude::document::eval(&format!(
            r#"setTimeout(function(){{var e=document.getElementById("{id}");if(e)e.scrollIntoView({{behavior:"smooth"}});}},50);"#
        ));
    }
}
