# vga9x16

A simple HTTP API for a very specific usecase:

- You need a randomly generated blob of ASCII-art.
- It needs to be unique for each client.
- You need it as PNG, for example to use as a CSS background.

## Usage

Run development server with `cargo run`. Docker support coming soon&trade;.

Navigate to `/` for an example background image usage. The `/png` route returns the sought-after ASCII-art blob.
