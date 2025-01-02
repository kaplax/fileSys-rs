use leptos::prelude::document;

pub fn mount_style(id: &str, content: &'static str) {
    let id = format!("thaw-id-{id}");

    let head = document().head().expect("head no exist");
    let style = head
        .query_selector(&format!("style#{id}"))
        .expect("query style element error");

    if style.is_some() {
        return;
    }

    let style = document()
        .create_element("style")
        .expect("create style element error");
    _ = style.set_attribute("id", &id);
    style.set_text_content(Some(content));
    _ = head.prepend_with_node_1(&style);
}
