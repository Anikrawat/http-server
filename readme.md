
# 🦀 Tiny HTTP Server in Rust

A minimal multithreaded HTTP server written **from scratch in pure Rust** (without frameworks).  
This project demonstrates low-level networking using `TcpListener`, multithreading with a thread pool, and basic HTTP parsing.

---

## ✨ Features
- `GET /` → Welcome message  
- `GET /echo/{msg}` → Echoes back `{msg}`  
- `GET /user-agent` → Returns the client **User-Agent** header  
- `GET /files/{filename}` → Serves files from the `tmp/` directory  
- `POST /files/{filename}` → Saves request body into `tmp/{filename}`  

---

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Cargo (comes with Rust)

### Run the server
```bash
git clone https://github.com/your-username/rust-tiny-http.git
cd rust-tiny-http
cargo run
