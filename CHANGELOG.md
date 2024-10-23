# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/timvw/chatbot-api/compare/v0.1.0...v0.1.1) - 2024-10-23

### Added

- add tracing span to openai call
- shutdown otel
- tracing to jaeger server
- make openai call
- update thread when message is added
- replace mutex with arcswap
- implement inmemory threads service
- graceful shutdown
- more work on exposing threads
- add tracing
- introduce sessions route
- added initial endpoint and introduce anyhow
- initial work to run a webserver

### Fixed

- *(deps)* update rust crate serde to v1.0.213
- *(deps)* update rust crate anyhow to v1.0.91
- *(deps)* update rust crate serde to v1.0.211
- *(deps)* update rust crate serde_json to v1.0.132
- *(deps)* update rust crate serde_json to v1.0.131
- *(deps)* update rust crate serde_json to v1.0.130
- *(deps)* update rust crate anyhow to v1.0.90
- *(deps)* update rust crate serde_json to v1.0.129
- *(deps)* update rust crate uuid to v1.11.0
- *(deps)* update rust crate async-openai to 0.25.0
- *(deps)* update rust crate async-openai to v0.24.1
- *(deps)* update rust crate init-tracing-opentelemetry to 0.22.0
- *(deps)* update rust crate tower-http to v0.6.1
- *(deps)* update rust crate axum-tracing-opentelemetry to v0.21.1
- *(deps)* update rust crate init-tracing-opentelemetry to 0.21.0
- *(deps)* update rust crate axum-tracing-opentelemetry to 0.21.0
- *(deps)* update rust crate tower-http to 0.6
- *(deps)* update rust crate tokio to v1.40.0
- *(deps)* update rust crate serde_json to v1.0.128
- *(deps)* update rust crate serde to v1.0.210
- *(deps)* update rust crate anyhow to v1.0.89
- *(deps)* update rust crate opentelemetry to 0.25.0
- *(deps)* update rust crate serde_json to v1.0.127
- *(deps)* update rust crate serde to v1.0.209
- add missing file

### Other

- describe env files
- make ThreadsState private
- remove comments
- remove unused settings for now
- added README file
- added repository to package
