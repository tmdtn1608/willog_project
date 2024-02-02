// Custom Responder.
use rocket::http::{ContentType, Header};

// #[derive(Responder)]
#[response(status = 200, content_type = "json")]
#[derive(rocket::response::Responder)]
pub struct MyResponder {
    inner: OtherResponder,
    // Override the Content-Type declared above.
    header: ContentType,
    more: Header<'static>,
    #[response(ignore)]
    unrelated: MyType,
}
