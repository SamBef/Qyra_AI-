# Qyra v2.5 — Project Roadmap

> **Stack:** Rust · Axum · Leptos · Tauri v2 · SurrealDB · Supabase Auth · LanceDB · Candle · Whisper-RS · LibP2P · Docker
> **Goal:** Africa-first, mobile-first, offline-capable personal AI agent

---

## Phase 0 — Foundation & Project Scaffold
*Get the monorepo set up, CI wired, and shared config in place.*

- [ ] **0.1** Initialise Rust workspace (`Cargo.toml` workspace with member crates)
- [ ] **0.2** Create crate skeleton: `qyra-relay`, `qyra-satellite`, `qyra-frontend`, `qyra-core`
- [ ] **0.3** Set up shared `config` module (ENV loading via `dotenvy`, `config` crate)
- [ ] **0.4** Set up logging (`tracing` + `tracing-subscriber`)
- [ ] **0.5** Docker Compose for local dev (SurrealDB + Redis/Upstash)
- [ ] **0.6** Basic CI pipeline (GitHub Actions: `cargo check`, `cargo clippy`, `cargo test`)
- [ ] **0.7** [.env.example](file:///c:/Users/User/Desktop/QYRA/openclaw/.env.example) with all required environment variables documented

---

## Phase 1 — Cloud Relay (Axum Backend)
*The "Tier 1" managed server. This is the first thing users connect to.*

### 1.1 — Database Layer
- [ ] **1.1.1** SurrealDB connection pool setup
- [ ] **1.1.2** Schema migrations: `user`, `linked_device`, `ananse_shard` tables (DOC 06)
- [ ] **1.1.3** Database abstraction layer (repository pattern)

### 1.2 — Authentication (Supabase Auth + JWT)
- [ ] **1.2.1** Supabase Auth integration (email/password + email verification)
- [ ] **1.2.2** Short-lived JWT middleware (15-min access tokens)
- [ ] **1.2.3** Rolling refresh token rotation (httpOnly cookie, 30-day TTL)
- [ ] **1.2.4** Rate limiting on auth endpoints (5 attempts/min, exponential backoff) (DOC 12.4)
- [ ] **1.2.5** Device fingerprinting on token issuance

### 1.3 — Gateway WebSocket
- [ ] **1.3.1** Axum WebSocket upgrade handler
- [ ] **1.3.2** Session management (create, resume, list, prune)
- [ ] **1.3.3** Message routing (inbound → agent → outbound)
- [ ] **1.3.4** Presence & typing indicators
- [ ] **1.3.5** Live Action Feed event emitter (structured log events per agent action) (DOC 13)

### 1.4 — AI Agent Core
- [ ] **1.4.1** LLM provider abstraction (supports Llama 3 via Groq/Together initially)
- [ ] **1.4.2** Tool/skill execution pipeline
- [ ] **1.4.3** Session context + pruning
- [ ] **1.4.4** Streaming response chunking

### 1.5 — Skill Packs (DOC 01 FR-05)
- [ ] **1.5.1** Skill registry data model
- [ ] **1.5.2** Trader Pack: MoMo balance check, price reply, inventory query
- [ ] **1.5.3** Student Pack: timetable, university portal navigation
- [ ] **1.5.4** Dev Pack: terminal (allowlisted), git ops, web search

### 1.6 — REST API Endpoints
- [ ] **1.6.1** `POST /auth/register` / `POST /auth/login` / `POST /auth/logout`
- [ ] **1.6.2** `GET /me` — user profile
- [ ] **1.6.3** `GET/POST /skills` — skill registry
- [ ] **1.6.4** `GET /sessions` — session history
- [ ] **1.6.5** `POST /agent/message` — send message to agent

---

## Phase 2 — Frontend (Leptos + DaisyUI)
*The web/WASM UI. Requires `cargo-leptos` to be installed.*

### 2.1 — Design System
- [ ] **2.1.1** Kente colour tokens (gold, ebony, kente-red, moss) (DOC 05)
- [ ] **2.1.2** Typography setup (Google Fonts — Inter or Outfit)
- [ ] **2.1.3** DaisyUI component overrides for Kente theme
- [ ] **2.1.4** Animated bot component (floating idle, gold active, purple biz) (DOC 05)

### 2.2 — Core Pages
- [ ] **2.2.1** Onboarding / registration page (Zero-config)
- [ ] **2.2.2** Email verification page
- [ ] **2.2.3** Dashboard / Chat UI with Live Action Feed sidebar (DOC 13)
- [ ] **2.2.4** Mode switcher (Cloud / Desktop / Home Node tiers)
- [ ] **2.2.5** Settings page (profile, language, skill packs, tier)
- [ ] **2.2.6** Skill Pack selection page

### 2.3 — Real-time Chat
- [ ] **2.3.1** WebSocket client (connect to Relay gateway)
- [ ] **2.3.2** Message list with streaming render
- [ ] **2.3.3** Typing indicator
- [ ] **2.3.4** Live Action Feed panel (collapsible, human-readable agent logs)
- [ ] **2.3.5** 5-second "Undo" window for reversible actions

---

## Phase 3 — Desktop Satellite (Tauri v2)
*The 10MB Windows/Mac binary that lets the phone control the PC.*

### 3.1 — Satellite Binary
- [ ] **3.1.1** Tauri v2 project setup in `qyra-satellite` crate
- [ ] **3.1.2** LibP2P peer-to-peer tunnel (NAT traversal) (DOC 04)
- [ ] **3.1.3** QR code pairing flow (mDNS local discovery) (DOC 07)
- [ ] **3.1.4** QR nonce expiry (120-second window) (DOC 12.3)
- [ ] **3.1.5** Certificate pinning after QR pairing
- [ ] **3.1.6** 4-emoji visual fingerprint confirmation

### 3.2 — Command Execution (Allowlist Only) (DOC 12.3)
- [ ] **3.2.1** Intent enum: `OPEN_URL`, `TYPE_TEXT`, `OPEN_FILE`, `FOCUS_WINDOW`, `SCREENSHOT`
- [ ] **3.2.2** Parameter validation per intent (URL scheme check, path sandbox)
- [ ] **3.2.3** OS sandbox (AppContainer on Windows, systemd `ProtectSystem` on Linux)
- [ ] **3.2.4** "Run Terminal Command" opt-in gate (disabled by default)

### 3.3 — Satellite Modes
- [ ] **3.3.1** Trader Mode preset (receipts, inventory, browser, WhatsApp Desktop)
- [ ] **3.3.2** Student Mode preset (documents, university portal, PDF summary)
- [ ] **3.3.3** Dev Mode preset (terminal allowlist, git, IDE integration)

### 3.4 — `linked_device` Registration
- [ ] **3.4.1** Register Satellite to `linked_device` table on pair
- [ ] **3.4.2** Online/offline/syncing status heartbeat
- [ ] **3.4.3** Encrypted P2P tunnel (or Relay fallback) for commands

---

## Phase 4 — Ananse Security (DOC 02 + DOC 12)
*The social recovery system — your "Village" protects your vault.*

### 4.1 — Key Management
- [ ] **4.1.1** `zeroize` crate integration on all key structs (`ZeroizeOnDrop`)
- [ ] **4.1.2** HKDF session key derivation (never use master key directly)
- [ ] **4.1.3** 500ms maximum master key lifetime in RAM (DOC 12.2)
- [ ] **4.1.4** AES-GCM local vault encryption for on-device shard

### 4.2 — Shamir's Secret Sharing (5-of-3 threshold)
- [ ] **4.2.1** `ssss` crate integration — 5 shards, 3 required to reconstruct
- [ ] **4.2.2** Shard distribution to Guardians (encrypted for each Guardian)
- [ ] **4.2.3** `ananse_shard` table storage (DOC 06)
- [ ] **4.2.4** Relationship diversity enforcement (Family / Friend / Colleague / Service) (DOC 12.1)

### 4.3 — Recovery Protocol
- [ ] **4.3.1** 24–48 hour mandatory time-lock on recovery initiation
- [ ] **4.3.2** Out-of-band SMS confirmation ("CONFIRM" / "CANCEL") (DOC 12.1)
- [ ] **4.3.3** Guardian UX: 10-second countdown + confirmation phrase required
- [ ] **4.3.4** Geographic diversity warning during Guardian setup
- [ ] **4.3.5** BLS-signed append-only audit log for all Guardian events (DOC 12.1)

### 4.4 — Transaction Signing
- [ ] **4.4.1** `threshold_crypto` BLS signatures for multi-guardian approval
- [ ] **4.4.2** Witness signature required for transactions > 500 units (DOC 02)

---

## Phase 5 — AI Intelligence Layer (DOC 04)
*Local-first AI inference — the "Home Node" brain.*

### 5.1 — LanceDB Memory
- [ ] **5.1.1** LanceDB setup for semantic long-term memory
- [ ] **5.1.2** Role-based memory namespace (Shared Agent: Owner/Manager/Staff/View) (DOC 13)
- [ ] **5.1.3** "One-tap" cloud-to-local memory migration tool (DOC 07)

### 5.2 — Candle Local Inference
- [ ] **5.2.1** `candle-core` integration for quantized LLM (Phi-3)
- [ ] **5.2.2** Offline fallback: switch to local Phi-3 when internet unavailable
- [ ] **5.2.3** Seamless tier switching (Cloud ↔ Local) (DOC 07)

### 5.3 — Offline Voice (Whisper-RS)
- [ ] **5.3.1** `whisper-rs` C++ bindings integration
- [ ] **5.3.2** Whisper Tiny model for offline STT
- [ ] **5.3.3** Language support hooks: Twi, Yoruba, Hausa, Swahili (fine-tuning pipeline) (DOC 01 FR-04)

---

## Phase 6 — Qyra Biz (DOC 01 FR-06 + DOC 12.5)
*Premium merchant features — the $12/mo revenue driver.*

- [ ] **6.1** WhatsApp Auto-Reply Agent (responds to customers 24/7)
- [ ] **6.2** Immutable system prompt prefix (non-overridable agent persona) (DOC 12.5)
- [ ] **6.3** Input sanitisation layer (strip prompt injection patterns)
- [ ] **6.4** Output validation classifier (no leaking prices/contacts)
- [ ] **6.5** Receipt OCR pipeline (magic bytes validation → re-encode → isolated worker)
- [ ] **6.6** Inventory Watch (camera-based stock counting)
- [ ] **6.7** Shared Agent roles (Owner / Manager / Staff / View-only) (DOC 13)

---

## Phase 7 — Home Node (Docker + Self-Hosting)
*The Raspberry Pi / Mini PC deployment path.*

- [ ] **7.1** Docker Compose file for full Home Node stack
- [ ] **7.2** Watchtower auto-updater container (DOC 12.6)
- [ ] **7.3** LUKS full-disk encryption setup guide (DOC 12.6)
- [ ] **7.4** Remote wipe command on node-stolen event
- [ ] **7.5** Shard rotation on compromise trigger
- [ ] **7.6** Mobile app version reporting (UP TO DATE / OUTDATED / CRITICAL badge)
- [ ] **7.7** `unattended-upgrades` OS security patch setup

---

## Phase 8 — Polish, QA & Launch Prep
- [ ] **8.1** Live Action Feed — full plain-language rendering for all agent actions (DOC 13)
- [ ] **8.2** African-native skill library: MoMo, GhanaPostGPS, NHIS, DSTV, Jumia (DOC 13)
- [ ] **8.3** Community skill library (published skills go into shared registry)
- [ ] **8.4** `toxiproxy` 2G/Edge network simulation tests (DOC 10)
- [ ] **8.5** Voice validation with Common Voice datasets (Accra/Lagos beta)
- [ ] **8.6** External security audit (ssss, libp2p, Ananse flow) — **hard gate** before v1.0 (DOC 12)
- [ ] **8.7** Freemium tier enforcement (50 cloud msgs/day cap for free users)
- [ ] **8.8** Qyra Core / Qyra+ / Qyra Biz billing integration

---

## Build Order Summary

```
Phase 0 → Phase 1 (Relay) → Phase 2 (Frontend) → Phase 3 (Satellite)
                                    ↓
                             Phase 4 (Ananse)
                                    ↓
                             Phase 5 (AI/Voice)
                                    ↓
                        Phase 6 (Biz) + Phase 7 (Home Node)
                                    ↓
                             Phase 8 (Launch)
```

> Phases 0–2 can be started **now** without `cargo-leptos`.
> Phase 2 (Frontend compilation) requires `cargo-leptos` to be installed first.
