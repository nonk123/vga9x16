#[macro_use]
extern crate rocket;

use std::{
    sync::{OnceLock, RwLock},
    time::Duration,
};

use image::{
    ExtendedColorType, GenericImage, GenericImageView, ImageEncoder, ImageReader, Rgb, RgbImage,
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

    rocket::build().mount("/", routes![index, favicon, robots, png])
}

#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open(if option_env!("VGA9X16_PUBLIC").is_some() {
        "index.public.html"
    } else {
        "index.html"
    })
    .await
    .unwrap()
}

#[get("/favicon.avif")]
async fn favicon() -> NamedFile {
    NamedFile::open("favicon.avif").await.unwrap()
}

#[get("/robots.txt")]
async fn robots() -> NamedFile {
    NamedFile::open("robots.txt").await.unwrap()
}

#[get("/png")]
async fn png() -> (ContentType, Vec<u8>) {
    let png = PNG.read().unwrap().as_ref().unwrap().clone();
    (ContentType::PNG, png)
}

const COLORS: [Rgb<u8>; 15] = [
    //Rgb([0x00, 0x00, 0x00]),
    Rgb([0x00, 0x00, 0xAA]),
    Rgb([0x00, 0xAA, 0x00]),
    Rgb([0x00, 0xAA, 0xAA]),
    Rgb([0xAA, 0x00, 0x00]),
    Rgb([0xAA, 0x00, 0xAA]),
    Rgb([0xAA, 0x55, 0x00]),
    Rgb([0xAA, 0xAA, 0xAA]),
    Rgb([0x55, 0x55, 0x55]),
    Rgb([0x55, 0x55, 0xFF]),
    Rgb([0x55, 0xFF, 0x55]),
    Rgb([0x55, 0xFF, 0xFF]),
    Rgb([0xFF, 0x55, 0x55]),
    Rgb([0xFF, 0x55, 0xFF]),
    Rgb([0xFF, 0xFF, 0x55]),
    Rgb([0xFF, 0xFF, 0xFF]),
];

const FORBIDDEN_GLYPHS: &[(u32, u32)] = &[
    (13, 11), // solid block
    (0, 8),   // almost solid
    (0, 10),
];

fn is_ok(glyph_idx: u32) -> bool {
    glyph_idx <= 255 && !FORBIDDEN_GLYPHS.contains(&(glyph_idx / 16, glyph_idx % 16))
}

fn next_glyph() -> u32 {
    let mut glyph_idx = 256;

    while !is_ok(glyph_idx) {
        glyph_idx = rand::rng().next_u32() % 256;
    }

    glyph_idx
}

fn next_color() -> usize {
    rand::rng().next_u32() as usize % COLORS.len()
}

fn generate() {
    let mut png = RgbImage::new(SIZE, SIZE);
    let font = FONT.get().unwrap();

    for out_y in 0..(SIZE / GLYPH_HEIGHT) {
        for out_x in 0..(SIZE / GLYPH_WIDTH) {
            let glyph_idx = next_glyph();
            let color_idx = next_color();

            let in_x = (glyph_idx % 16) * GLYPH_WIDTH;
            let in_y = (glyph_idx / 16) * GLYPH_HEIGHT;

            let out_x = out_x * GLYPH_WIDTH;
            let out_y = out_y * GLYPH_HEIGHT;

            let mut glyph = font.view(in_x, in_y, GLYPH_WIDTH, GLYPH_HEIGHT).to_image();

            for pixel in glyph.pixels_mut() {
                if *pixel == Rgb([0xFF, 0xFF, 0xFF]) {
                    *pixel = COLORS[color_idx];
                }
            }

            png.copy_from(&glyph, out_x, out_y).unwrap();
        }
    }

    let mut buffer = vec![];
    PngEncoder::new(&mut buffer)
        .write_image(&png, png.width(), png.height(), ExtendedColorType::Rgb8)
        .unwrap();
    PNG.write().unwrap().replace(buffer);
}
