use gloo::events::EventListener;
use gloo::utils::document;

pub fn main() {
    let _listener = mainbody_listener();
}

fn mainbody_listener() -> EventListener {
    let document = document();
    let mainbody = document.get_element_by_id("mainbody").unwrap();
    EventListener::new(&mainbody, "click", |_event| {})
}
