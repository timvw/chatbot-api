# Chatbot-API

A demo application exposing a chatbot API build with Rust ([axum](https://github.com/tokio-rs/axum) + friends)

## Configuration

Create a dotenvs/openai.env file with the following keys:

```text
OPENAI_API_KEY="sk-xxxx"
```

Create a dotenvs/jaeger.env file with the following keys:

```text
OTEL_EXPORTER_OTLP_TRACES_ENDPOINT=http://yourhost:4317
OTEL_EXPORTER_OTLP_PROTOCOL=grpc
OTEL_TRACES_SAMPLER=always_on
OTEL_SERVICE_NAME=chatapi
```

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