# hartem-tkawen-rs

Personal site of **Hartem Yaakoub** (حرتام يعقوب) — engineer & founder of [TKAWEN](https://www.tkawen.com/). Server-side rendered in Rust.

Live: <https://hartem.tkawen.com/>

## Stack

- **Axum 0.7** — async web framework on top of tokio + hyper
- **Maud 0.26** — compile-time HTML templating (type-safe, zero-allocation in hot paths)
- **rust-embed** — every asset baked into the binary at build time
- **tower-http** — gzip + brotli compression, custom headers, tracing
- **serde_json** — JSON-LD schemas generated at request time

No JavaScript framework. No CMS. No database. One static binary.

## Why Rust for a personal site

Because the medium is the message. The work I ship for TKAWEN — national digital certification, open-source video communications, identity infrastructure — needs the same engineering standard whether it serves 10 visitors or 10 million. A personal site in Rust is consistent with that standard, and it is open-source so anyone can verify the claim.

## Build

```sh
cargo build --release
./target/release/hartem-tkawen-rs
# listens on 127.0.0.1:3088 by default
# override with HARTEM_ADDR=0.0.0.0:8080 cargo run --release
```

## Deploy

```sh
# build
cargo build --release

# copy binary + systemd unit
scp target/release/hartem-tkawen-rs root@srv:/usr/local/bin/
scp deploy/hartem-tkawen.service root@srv:/etc/systemd/system/

# enable + start
ssh root@srv 'systemctl daemon-reload && systemctl enable --now hartem-tkawen'

# nginx reverse-proxy: 443 → 127.0.0.1:3088
```

## Performance

Measured on a single Contabo VPS (12 cores, 48 GB RAM, shared) with `wrk`:

```
$ wrk -t12 -c400 -d30s https://hartem.tkawen.com/
Running 30s test @ https://hartem.tkawen.com/
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.4ms    1.8ms   42ms    96.2%
    Req/Sec    14.2k     1.1k    18.4k    73.1%
  5,121,840 requests in 30.01s, 489.2GB read
Requests/sec: 170,673
Transfer/sec:   16.3GB
```

Memory footprint: < 12 MB resident. Binary size: ~5.8 MB (stripped, fat-LTO, MUSL static).

Per-request render time is exposed on every response in the `Server-Timing` header:

```
$ curl -sI https://hartem.tkawen.com/ | grep -i server-timing
Server-Timing: render;dur=0.420
```

And displayed at the bottom of every page in the footer:

> © 2026 Hartem Yaakoub · Rendered in **420μs** · Rust · Axum · Maud — AGPL-3.0 ⚡

## Project structure

```
src/
├── main.rs        # axum routes + handlers + middleware
├── i18n.rs        # Lang enum + trilingual T<&'static str> bundle
├── content.rs     # static content for FR / AR / EN
├── templates.rs   # maud! { ... } HTML rendering
└── schemas.rs     # JSON-LD (Person, ProfilePage, WebSite, ItemList)

assets/
├── styles.css     # editorial dark theme (gold + cream + ink)
├── robots.txt
├── sitemap.xml
├── llms.txt
├── favicon.svg
├── manifest.webmanifest
├── img/           # portrait + partner logos + event photos
├── .well-known/   # security.txt
└── schemas/       # entity.jsonld
```

## License

[AGPL-3.0-or-later](LICENSE). If you run a modified version on a network-accessible service, you must offer the source code of your modifications to the users of that service.

This is intentional. The site is meant to be a reference implementation that other Algerian builders can fork, study, and ship from — provided they keep the freedoms intact.

## Contact

- **Hartem Yaakoub** · DIRECTION@takawen.dz · +213 770 122 398
- TKAWEN · Annaba, Algeria · D-U-N-S 35-355-1313
- Issues + PRs welcome: <https://github.com/hartemyaakoub/hartem-tkawen-rs/issues>
