use web_sys::window;

mod hostkey;

fn main() {
    console_error_panic_hook::set_once();

    let host_key = hostkey::HostKey::new();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node(&format!(
        "HostKey: public_key: {}, seed: {}",
        host_key.public_key(),
        host_key.seed()
    ));
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
}
