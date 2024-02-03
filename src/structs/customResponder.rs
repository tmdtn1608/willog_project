// Custom Responder.
use rocket::http::{ContentType, Header};

#[response(status = 200, content_type = "json")]
#[derive(rocket::response::Responder)]
pub struct MyResponder {
    inner: OtherResponder,
    header: ContentType,
    more: Header<'static>,
    #[response(ignore)]
    unrelated: MyType,
}
