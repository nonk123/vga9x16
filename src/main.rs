#[macro_use]
extern crate rocket;

use std::{
    sync::{OnceLock, RwLock},
    time::Duration,
};

use image::{
    ExtendedColorType, GenericImage, GenericImageView, ImageEncoder, ImageReader, RgbImage,
    codecs::png::PngEncoder,
};
use rand::RngCore;
use rocket::{fs::NamedFile, http::ContentType};

const UPDATE_INTERVAL: f32 = 1.0;

const SIZE: u32 = 288;
const GLYPH_WIDTH: u32 = 9;
const GLYPH_HEIGHT: u32 = 16;

static FONT: OnceLock<RgbImage> = OnceLock::new();
static PNG: RwLock<Option<Vec<u8>>> = RwLock::new(None);

#[launch]
async fn rocket() -> _ {
    FONT.set(
        ImageReader::open("9x16.png")
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8(),
    )
    .unwrap();

    generate();
    std::thread::spawn(|| {
        loop {
            std::thread::sleep(Duration::from_secs_f32(UPDATE_INTERVAL));
            generate();
        }
    });

    rocket::build().mount("/", routes![index, favicon, png])
}

#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open("index.html").await.unwrap()
}

#[get("/favicon.avif")]
async fn favicon() -> NamedFile {
    NamedFile::open("favicon.avif").await.unwrap()
}

#[get("/png")]
async fn png() -> (ContentType, Vec<u8>) {
    let png = PNG.read().unwrap().as_ref().unwrap().clone();
    (ContentType::PNG, png)
}

fn generate() {
    let mut png = RgbImage::new(SIZE, SIZE);
    let font = FONT.get().unwrap();

    for out_y in 0..(SIZE / GLYPH_HEIGHT) {
        for out_x in 0..(SIZE / GLYPH_WIDTH) {
            let idx = rand::rng().next_u32() % 256;
            let in_x = (idx % 16) * GLYPH_WIDTH;
            let in_y = (idx / 16) * GLYPH_HEIGHT;

            let out_x = out_x * GLYPH_WIDTH;
            let out_y = out_y * GLYPH_HEIGHT;

            let glyph = font.view(in_x, in_y, GLYPH_WIDTH, GLYPH_HEIGHT).to_image();
            png.copy_from(&glyph, out_x, out_y).unwrap();
        }
    }

    let mut buffer = vec![];
    PngEncoder::new(&mut buffer)
        .write_image(&png, png.width(), png.height(), ExtendedColorType::Rgb8)
        .unwrap();
    PNG.write().unwrap().replace(buffer);
}
