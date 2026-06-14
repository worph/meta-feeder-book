# meta-feeder-book

Book **feeder sidecar** for [MetaMesh](https://github.com/worph/meta-gateway) —
surfaces public-domain books from **Project Gutenberg** (via the
[Gutendex](https://gutendex.com) API) into a meta-gateway.

It is the proof-of-concept feeder: a single upstream, no preview-seeding, no
BitTorrent — just enough to exercise the `meta-feeder-sdk` + feeder HTTP
contract end to end.

## Role in MetaMesh

A feeder is a stateless HTTP sidecar. It **finds records and fetches bytes**; it
does *not* talk to meta-core or the libp2p blockstore. The gateway core that
calls it owns the meta-core store-back and the blockstore seeding. A gateway
registers this feeder as a `RemoteFeederPlugin` pointing at its `/` and then
drives the contract:

| Endpoint | Purpose |
|----------|---------|
| `GET /manifest` | feeder identity + capabilities |
| `GET /health` | liveness |
| `POST /query`, `POST /query_stream` | structured search against the upstream |
| `POST /compute` | enrichment / outcome compute |
| `GET /fetch/:upstream_id/:record_id` | fetch a record's bytes |
| `GET /blob/:upstream_id/:cid` | fetch a content-addressed blob |
| `GET /config`, `GET /config/schema`, `GET\|PUT /config/values` | runtime config UI + API |

## Configuration

| Env var | Default | Notes |
|---------|---------|-------|
| `META_FEEDER_HTTP_LISTEN` | `0.0.0.0:8080` | HTTP listen address |
| `META_FEEDER_STATE_DIR` | `/data/meta-feeder` | redb cache + state |
| `RUST_LOG` | `info` | tracing filter |

No upstream API key is required — Gutendex is open.

## Image

```
ghcr.io/worph/meta-feeder-book
```

Exposes `8080`. Built and pushed by CI on every push to `main` (the `main`
tag) and on `v*` tags (semver tags).

## Build locally

The build context is the **repo root** (the Cargo workspace) so the vendored
`meta-feeder-sdk` path dependency resolves:

```bash
docker build -f feeder-plugin/book-feeder/Dockerfile -t ghcr.io/worph/meta-feeder-book:dev .
```

## Repo layout

This repo is a self-contained Cargo workspace vendored out of the
`meta-gateway` monorepo:

```
Cargo.toml                      # workspace: members = crates/*, feeder-plugin/*
crates/meta-feeder-sdk/         # vendored shared feeder SDK
feeder-plugin/book-feeder/      # this feeder's crate + Dockerfile
```

Upstream source of truth for the SDK and the feeder crate is
[`worph/meta-gateway`](https://github.com/worph/meta-gateway); changes there are
vendored back into this repo.
