# Synapse

An AI agent framework for LLM infrastructure, written in Rust. Synapse lets you build agents that talk to **any LLM provider**, connect to **any external platform**, and — critically — translate natural language into accurate, executable **database commands** across any database engine.

---

## Why Synapse?

Most agent frameworks make you pick a provider, wire up your own connectors, and then fumble through prompt engineering whenever a user asks something that touches a database. Synapse is built around the opposite philosophy:

- **Provider-agnostic** — swap between OpenAI, Anthropic, GitHub Copilot, or your own endpoint in one line.
- **Platform-agnostic** — a uniform connector model lets agents reach Slack, GitHub, REST APIs, message queues, or any other external system.
- **Database-first** — the agent's reasoning layer understands schemas, dialects, and query semantics so that "show me last month's revenue by region" becomes a correct SQL (or Cypher, MQL, …) query, not a hallucination.

---

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                        Outer Scope: Life                     │
│                                                              │
│      state -> state -> state -> ...                          │
│                                                              │
│        living -> think -> interact -> update_state           │
└──────────────────────────────────────────────────────────────┘
                           │
                           ▼
                   Brain: Cortex::think
                (provider output -> Thought)
```

### Core modules

| Module | Description |
|---|---|
| `agent::brain::cortex` | Think-only brain core. Converts perceived messages into a `Thought`. |
| `agent::brain::memory` | Conversation and working memory. Stores, reads, and retrieves message history. |
| `agent_loop` | Defines `Life` with `internal_state` + `external_state`, and drives `living()`. |
| `agent::planning` | Reserved scaffold (not in active execution path right now). |
| `agent::action` | Reserved scaffold for future action/tool routing. |
| `providers` | Pluggable LLM backends (Copilot, OpenAI, Anthropic, …). |
| `platforms` | Connectors to external systems (databases, APIs, messaging, …). |
| `skills` | Reusable capabilities the agent can invoke (query builder, code runner, …). |

---

## Current Loop Model

The runtime model is intentionally minimal and explicit:

`Life::living -> think -> interact -> update_state -> repeat`

- **State container** lives in `Life` as two entities:
        - `internal_state` (currently includes `Memory`)
        - `external_state`
- **Think** produces a `Thought` from the current runtime context.
- **Interact** applies that thought to the world boundary.
- **UpdateState** applies post-interaction mutations to state.

The loop itself lives at outer scope so callers can spawn/run an agent lifecycle directly, while brain internals remain isolated.

---

## Adaptive Intelligence Roadmap

Synapse already has a strong architectural skeleton. The next evolution is to turn it into a continuously adaptive agent that behaves more like a living collaborator: aware of uncertainty, able to self-correct, and able to improve over time.

### Current strengths

- Clean separation of concerns across `cortex`, `memory`, `planning`, `action`, and `runtime`.
- Support for both interactive and programmatic execution modes.
- Early tool discovery and runtime policy primitives already in place.

### What to add next

- **Self-model**: track internal state such as goals, confidence, constraints, and budget.
- **Metacognition loop**: evaluate each action for usefulness/correctness/safety, then adjust strategy.
- **Memory curation**: move beyond append-only history into semantic + episodic memory with confidence and decay.
- **Dynamic replanning**: trigger replans on tool failures, contradictory evidence, or low-confidence outcomes.
- **Uncertainty behavior**: explicitly represent "not enough evidence yet" and gather more context before concluding.

### Core adaptive loop (target behavior)

`Living -> Think -> Interact -> UpdateState`

This loop runs continuously and treats the system as a state machine over two entities: external world state and AI self state.

### Prioritized implementation plan

1. Keep `Cortex` think-only and stateless with respect to outer control flow.
2. Evolve `Life` state transitions from TODO signatures into concrete implementations.
3. Add structured LLM turn schema parsing (thought/action/evidence payload).
4. Add tool execution bridge in `interact` and feed results back into memory/state.
5. Enforce safety and budgets in `agent::runtime::{policy,sandbox}` for all interactions.
6. Add evaluation harnesses for regression tracking (task success, correction rate, hallucination rate).

These changes preserve Synapse's provider/platform/database-agnostic design while making the agent significantly more reliable, self-correcting, and "alive" in practice.

---

## LLM Providers

Synapse ships with first-party provider support and a simple trait you can implement for anything else.

| Provider | Status |
|---|---|
| GitHub Copilot | ✅ OAuth device-flow, automatic token refresh, disk caching |
| OpenAI | 🚧 In progress |
| Anthropic | 🚧 In progress |
| Custom / self-hosted | ✅ Implement the `Provider` trait |

The Copilot provider authenticates via GitHub's device-flow — the same mechanism the VS Code extension uses — so no API key management is required.

---

## Database Intelligence

Synapse treats databases as first-class citizens, not afterthoughts. The agent:

1. **Introspects schemas** at connection time so it understands table structures, types, and relationships.
2. **Translates intent** — the Cortex converts a user's natural language request into a precise query in the correct dialect (SQL, Cypher, MQL, DynamoDB expressions, …).
3. **Validates before executing** — the planned query is checked for safety and correctness before it ever touches the database.
4. **Explains results** — raw results are interpreted back into natural language with the original question in mind.

Planned database targets:

- Relational: PostgreSQL, MySQL, SQLite
- Document: MongoDB
- Graph: Neo4j
- Key-value / wide-column: Redis, Cassandra, DynamoDB
- Vector: pgvector, Qdrant, Weaviate

---

## Getting Started

### Prerequisites

- Rust 1.75+
- Cargo

### Build

```bash
git clone https://github.com/your-org/synapse
cd synapse
cargo build --release
```

### Run

```bash
cargo run
```

### Async Runtime

Synapse uses **Tokio** as its async runtime.

- Core modules expose async signatures.
- Runtime driving/execution happens at the application boundary (entrypoint / runner).

---

## Project Status

Synapse is in early active development. The core `Brain` (Cortex + Memory), the Copilot provider, and the module skeleton are in place. Provider abstraction, the planning loop, database connectors, and the skills system are being built out now.

Contributions, feedback, and use-case ideas are welcome — open an issue or a PR.

---

## License

MIT
