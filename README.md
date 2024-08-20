# Chatbot-API

A demo application exposing a chatbot API build with Rust ([axum](https://github.com/tokio-rs/axum) + friends)

## Development

```bash
cargo watch -q -c -w src/ -x run
```

```bash
curl -i http://localhost:3000/threads
```

```bash
curl -X POST -H "Content-type: application/json" -i -vvv http://localhost:3000/threads -d ''
```

```bash
curl -i http://localhost:3000/threads/67e55044-10b1-426f-9247-bb680e5fe0c8/messages
```

```bash
curl -X POST -H "Content-type: application/json" -i http://localhost:3000/threads/67e55044-10b1-426f-9247-bb680e5fe0c8/messages -d '{"content": "awesome"}'
```