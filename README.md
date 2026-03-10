# Qyra v2.5 — Africa-First AI Agent

> **Intelligence. Redefined. — Zero Config. Africa-First. Unstoppable.**

Qyra is a Pan-African AI agent built in Rust. It combines the instant utility of cloud agents with the offline sovereignty of a local node — mobile-first, community-trusted, and built for the realities of African connectivity.

## Tech Stack

| Layer | Technology |
|---|---|
| Frontend | Leptos (Rust/WASM) + DaisyUI (Kente theme) |
| Desktop | Tauri v2 (Windows/Mac Satellite) |
| Backend | Axum (Cloud Relay) |
| Database | SurrealDB |
| Auth | Supabase Auth + JWT |
| Memory | LanceDB (vector store) |
| AI | Candle (Phi-3 local) + Llama 3 (cloud) |
| Voice | Whisper-RS (offline — Twi, Yoruba, Hausa, Swahili) |
| P2P | LibP2P (Desktop Link) |
| Security | Shamir's SSS (Ananse Protocol) + AES-GCM |

## Getting Started

### Prerequisites
- Rust 1.75+ (`rustup`)
- Docker & Docker Compose (for local services)
- Node.js 22+ (for frontend tooling)
- `cargo-leptos` (for frontend — install when available)

### Local Development

```bash
# 1. Clone the repo
git clone https://github.com/SamBef/Qyra_AI-.git
cd Qyra_AI-

# 2. Copy environment config
cp .env.example .env
# Fill in your values in .env

# 3. Start local services (SurrealDB + Redis)
docker compose up -d

# 4. Run the relay server
cargo run -p qyra-relay
```

### Project Structure

```
QYRA/
├── crates/
│   ├── qyra-core/        # Shared types, config, models
│   ├── qyra-relay/       # Cloud Relay (Axum HTTP + WebSocket server)
│   ├── qyra-satellite/   # Desktop Satellite (Tauri v2 binary)
│   └── qyra-frontend/    # Web UI (Leptos WASM)
├── openclaw/             # Competitor benchmark (reference only)
├── skills/               # Skill SDK spec and templates
├── docker-compose.yml    # Local dev services
└── .env.example          # Environment variable template
```

## Roadmap

See [qyra_roadmap.md](qyra_roadmap.md) for the full phased build plan.

## License

UNLICENSED — Proprietary.
