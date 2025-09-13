
# 🦀 Tiny HTTP Server in Rust

A minimal **multithreaded HTTP server** built entirely in **pure Rust**, without using external web frameworks.  
It demonstrates low-level networking with `TcpListener`, concurrency with a thread pool, and manual HTTP parsing.  

---

## ✨ Features

- **Root** → `GET /`  
  Returns a welcome message.  

- **Echo** → `GET /echo/{msg}`  
  Returns back the `{msg}` you provide.  

- **User-Agent** → `GET /user-agent`  
  Extracts and returns the client `User-Agent` header.  

- **File Download** → `GET /files/{filename}`  
  Reads a file from the `tmp/` directory and returns its contents.  

- **File Upload** → `POST /files/{filename}`  
  Saves the request body into `tmp/{filename}`.  

---

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### Clone & Run
```bash
git clone https://github.com/your-username/rust-tiny-http.git
cd rust-tiny-http
cargo run
