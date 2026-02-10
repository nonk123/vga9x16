use std::io::Cursor;

use rocket::{
    Request, Response,
    http::{ContentType, Header},
    response::{self, Responder},
};

pub struct AsciiBlob(pub Vec<u8>);

#[rocket::async_trait]
impl<'r, 'o: 'r> Responder<'r, 'o> for AsciiBlob {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'o> {
        let caching = "public, no-store, max-age=1, s-maxage=1";
        Response::build()
            .header(ContentType::PNG)
            .header(Header::new("Cache-Control", caching))
            .header(Header::new("Access-Control-Allow-Methods", "GET"))
            .header(Header::new("Access-Control-Allow-Origin", "*"))
            .sized_body(self.0.len(), Cursor::new(self.0))
            .ok()
    }
}
