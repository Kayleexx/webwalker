# WebWalker

**WebWalker** is a blazing-fast, asynchronous web crawler written in Rust. It supports recursive crawling, rate-limiting, domain scoping, and output to JSON or text files.

## 🚀 Features

- Async crawling using `tokio` and `reqwest`
- HTML parsing via `scraper`
- Depth-limited recursive crawling
- Optional same-domain restriction
- Rate-limiting with `Semaphore`
- Timeout handling per request
- Output URLs to `.json` or `.txt`

## 📦 Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/Kayleexx/webwalker.git
cd webwalker
cargo build --release
````

## 🕸️ Usage

```bash
cargo run -- <URL> [OPTIONS]
```

### Required

* `<URL>`: Starting point for crawling.

### Options

| Flag            | Description                          | Default      |
| --------------- | ------------------------------------ | ------------ |
| `--depth`       | Maximum crawl depth                  | `2`          |
| `--rate`        | Max concurrent requests (semaphore)  | `10`         |
| `--timeout`     | Timeout per request (in seconds)     | `10`         |
| `--same-domain` | Restrict to same domain only         | `false`      |
| `--out`         | Output file path (`.json` or `.txt`) | *(optional)* |

### Example

```bash
cargo run -- https://example.com --depth 3 --rate 5 --timeout 15 --same-domain --out results.json
```

## 📁 Output

If `--out` is specified:

* `.json` → Saves URLs as a pretty JSON array
* `.txt`  → Saves URLs line-by-line

## 🔧 Development

```bash
cargo fmt     # format code
cargo clippy  # lint code
cargo test    # run tests
```

## working output

https://github.com/user-attachments/assets/d52b6f7f-6d92-4af2-a162-92512d5902d3

<img width="1327" height="956" alt="image" src="https://github.com/user-attachments/assets/776ea951-c1fc-4c7e-bf9a-fa8243bf6cd4" />


