# Chatbot-API

A demo application exposing a chatbot API build with Rust ([axum](https://github.com/tokio-rs/axum) + friends)

## Development

```bash
cargo watch -q -c -w src/ -x run
```

```bash
 curl -i -v http://localhost:3000/threads
```

```bash
curl -X POST -H "Content-type: application/json" -i -vvv http://localhost:3000/threads -d ''
```