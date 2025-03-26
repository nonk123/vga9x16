# vga9x16

A simple HTTP API serving a very specific usecase:

- You need a randomly generated blob of Dwarf Fortress styled ASCII-art.
- It needs to be somewhat unique for each request.
- It has to be in PNG format so you can use as a CSS background.

## Usage

Run the development server with `cargo run`.

A [Dockerfile](Dockerfile) is included for easy deployment. GitHub package coming soon&trade;.

Navigate to `/` for an example background image usage. The `/png` route returns the sought-after ASCII-art blob.

## Under the hood

Here are some implementation details for the curious:

- The ASCII-art PNG is updated every second. Two users reaching `/png` at around the same time will get duplicate blobs.
- You can set `VGA9X16_PUBLIC=1` at build time to return [index.public.html](index.public.html) for the `/` route.
