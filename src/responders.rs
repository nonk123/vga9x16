use rocket::{
    Request,
    http::Header,
    response::{self, Responder},
};

pub struct AsciiBlob<D>(pub D);

#[rocket::async_trait]
impl<'r, 'o: 'r, D: Responder<'r, 'o>> Responder<'r, 'o> for AsciiBlob<D> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        let mut response = self.0.respond_to(req)?;
        let caching = "public, no-store, max-age=1, s-maxage=1";
        response.set_header(Header::new("Cache-Control", caching));
        response.set_header(Header::new("Content-Type", "image/png"));
        Ok(response)
    }
}
