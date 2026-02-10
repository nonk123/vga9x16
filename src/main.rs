#[macro_use]
extern crate rocket;

use std::{
    sync::{OnceLock, RwLock},
    thread,
    time::Duration,
};

use color_eyre::eyre;
use image::{
    ExtendedColorType, GenericImage, GenericImageView, ImageEncoder, ImageReader, Rgba, RgbaImage,
    codecs::png::PngEncoder,
};
use rand::Rng;
use rocket::fs::NamedFile;

mod responders;

use responders::AsciiBlob;

const UPDATE_INTERVAL: f32 = 1.0;

const SIZE: u32 = 288;
const GLYPH_WIDTH: u32 = 9;
const GLYPH_HEIGHT: u32 = 16;

struct PngBlobs {
    classic: Vec<u8>,
    transparent: Vec<u8>,
}

static FONT: OnceLock<RgbaImage> = OnceLock::new();
static PNG: RwLock<Option<PngBlobs>> = RwLock::new(None);

#[rocket::main]
async fn main() -> eyre::Result<()> {
    let _ = color_eyre::install();
    {
        let img = ImageReader::open("9x16.png")?.decode()?.into_rgba8();
        FONT.set(img).unwrap();
    }

    generate_blobs();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs_f32(UPDATE_INTERVAL));
            generate_blobs();
        }
    });

    rocket::build()
        .mount("/", routes![index, favicon, robots, png])
        .launch()
        .await?;

    Ok(())
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

#[get("/png?<transparent>")]
async fn png(transparent: bool) -> AsciiBlob {
    let guard = PNG.read().unwrap();
    let pngs = guard.as_ref().unwrap();
    AsciiBlob(if transparent {
        pngs.transparent.clone()
    } else {
        pngs.classic.clone()
    })
}

const COLORS: [Rgba<u8>; 15] = [
    Rgba([0x00, 0x00, 0xAA, 0xFF]),
    Rgba([0x00, 0xAA, 0x00, 0xFF]),
    Rgba([0x00, 0xAA, 0xAA, 0xFF]),
    Rgba([0xAA, 0x00, 0x00, 0xFF]),
    Rgba([0xAA, 0x00, 0xAA, 0xFF]),
    Rgba([0xAA, 0x55, 0x00, 0xFF]),
    Rgba([0xAA, 0xAA, 0xAA, 0xFF]),
    Rgba([0x55, 0x55, 0x55, 0xFF]),
    Rgba([0x55, 0x55, 0xFF, 0xFF]),
    Rgba([0x55, 0xFF, 0x55, 0xFF]),
    Rgba([0x55, 0xFF, 0xFF, 0xFF]),
    Rgba([0xFF, 0x55, 0x55, 0xFF]),
    Rgba([0xFF, 0x55, 0xFF, 0xFF]),
    Rgba([0xFF, 0xFF, 0x55, 0xFF]),
    Rgba([0xFF, 0xFF, 0xFF, 0xFF]),
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

fn generate_blobs() {
    PNG.write().unwrap().replace(PngBlobs {
        classic: generate_blob(false),
        transparent: generate_blob(true),
    });
}

fn generate_blob(transparent: bool) -> Vec<u8> {
    let mut image = RgbaImage::new(SIZE, SIZE);

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
                if let Rgba([0xFF, 0xFF, 0xFF, _]) = *pixel {
                    *pixel = COLORS[color_idx];
                } else if transparent {
                    *pixel = Rgba([255, 255, 255, 0])
                } else {
                    *pixel = Rgba([0, 0, 0, 255])
                };
            }

            image.copy_from(&glyph, out_x, out_y).unwrap();
        }
    }

    let mut data = vec![];
    PngEncoder::new(&mut data)
        .write_image(
            &image,
            image.width(),
            image.height(),
            ExtendedColorType::Rgba8,
        )
        .unwrap();
    data
}
