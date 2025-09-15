# vga9x16

[See the live demo](https://vga9x16.ru).

A simple HTTP API serving a very specific usecase:

- You need a randomly generated blob of Dwarf Fortress styled ASCII-art.
- It needs to be somewhat unique for each request.
- It has to be in PNG format so you can use it as a CSS background.

## Usage

Run the development server with `cargo run`.

For deployment, use this sample `docker-compose.yml`; then point your reverse-proxy ([I recommend caddy](https://caddyserver.com/)) to `vga9x16:8000`:

```yaml
services:
  main:
    container_name: vga9x16
    image: ghcr.io/nonk123/vga9x16:release
    restart: always
    networks:
      - add your reverse-proxy network here...
```

Navigate to `/` for an example background image usage. The `/png` route returns the sought-after ASCII-art blob. Pass `?transparent=true` for a transparent background instead of black.

## Under the hood

Here are some implementation details for the curious:

- The ASCII-art PNG is updated every second. Two users reaching `/png` at around the same time will get duplicate blobs.
- You can set `VGA9X16_PUBLIC=1` at build time to return [index.public.html](index.public.html) for the `/` route.

## Attribution

Links to where the assets were borrowed from:

- [9x16.png](https://dwarffortresswiki.org/index.php/Tileset_repository#9.C3.9716).
- [favicon.avif](favicon.avif) - trimmed from [9x16.png](9x16.png) and reexported to AVIF.
