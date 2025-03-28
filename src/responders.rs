use rocket::{
    Request,
    http::Header,
    response::{self, Responder},
};

pub struct AsciiBlob<D>(pub D, pub &'static str);

#[rocket::async_trait]
impl<'r, 'o: 'r, D: Responder<'r, 'o>> Responder<'r, 'o> for AsciiBlob<D> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        let mut response = self.0.respond_to(req)?;
        response.set_header(Header::new("Cache-Control", "public, no-store"));
        response.set_header(Header::new("Content-Type", self.1));
        Ok(response)
    }
}
