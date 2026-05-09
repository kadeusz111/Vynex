# Vynex
Vynex is a lightweight local search engine written in Rust. It aggregates real-time results from multiple sources such as Wikipedia, GitHub, Reddit, YouTube (via Invidious), and DuckDuckGo, then merges them into a single ranked result page with support for custom priority links.

---

## 🚀 Features

- Real-time multi-source search
- Sources:
  - Wikipedia
  - GitHub
  - Reddit
  - YouTube (Invidious)
  - DuckDuckGo (fallback/temporary)
- Custom priority links (default results)
- Simple ranking system
- Temporary in-memory processing (no persistent cache)
- Lightweight Rust backend

---

## 🧠 How it works

1. User enters a query
2. Backend sends parallel requests to multiple sources
3. Results are scraped and parsed
4. Default priority links are added first (if matching query)
5. Results are ranked and merged
6. Final list is returned as JSON

---

## ⚙️ Requirements

- Rust (latest stable recommended)
- Cargo

---

## 🛠️ Build & Run

1. Clone repository
```bash
git clone https://github.com/kadeusz111/Vynex.git
cd Vynex/backend
```
2. Run backend
```bash
cargo run
```
Server will start on:
```bash
http://localhost:3000
```
3. Open frontend
Just open:
```bash
frontend/index.html
```
or use a local server

---

## 🔌 API

Search endpoint
```
GET /search?q=your_query
```
For example
```
http://localhost:3000/search?q=rust
```
Response
```json
[
  {
    "title": "Rust Programming Language",
    "url": "https://www.rust-lang.org",
    "snippet": "Official website",
    "source": "Wikipedia",
    "score": 80
  }
]
```

## ⭐ Default links
Custom high-priority links are stored in:
```
default_links.json
```
For example
```json
[
  {
    "keyword": "google",
    "title": "Google",
    "url": "https://google.com",
    "priority": 100
  }
]
```
⚠️ Notes
This project scrapes public web pages — some sources may block requests
DuckDuckGo and other sources may change HTML structure
YouTube scraping uses Invidious (recommended for stability)

## 📌TODO
- get rid of duckduckgo
- add more sites to scrap




