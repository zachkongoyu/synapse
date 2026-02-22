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
┌─────────────────────────────────────────────────────┐
│                      Agent                          │
│                                                     │
│  ┌──────────┐   ┌────────────┐   ┌───────────────┐  │
│  │  Brain   │   │  Planning  │   │    Actions    │  │
│  │          │   │            │   │               │  │
│  │ Cortex   │   │  Tasks &   │   │  Skills       │  │
│  │ Memory   │   │  State     │   │  Tool calls   │  │
│  └────┬─────┘   └─────┬──────┘   └──────┬────────┘  │
└───────┼───────────────┼─────────────────┼───────────┘
        │               │                 │
   LLM Providers   Planning loop     Platforms & DBs
  (OpenAI, Copilot,  (ReAct / ToT)   (Postgres, Mongo,
   Anthropic, …)                      Slack, REST, …)
```

### Core modules

| Module | Description |
|---|---|
| `agent::brain::cortex` | The reasoning core. Wraps any `Provider` and drives the think → act loop. |
| `agent::brain::memory` | Conversation and working memory. Stores, reads, and retrieves message history. |
| `agent::planning` | Decomposes goals into ordered task lists and tracks execution state. |
| `agent::action` | Dispatches tool calls returned by the LLM to the right skill or connector. |
| `providers` | Pluggable LLM backends (Copilot, OpenAI, Anthropic, …). |
| `platforms` | Connectors to external systems (databases, APIs, messaging, …). |
| `skills` | Reusable capabilities the agent can invoke (query builder, code runner, …). |

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

---

## Project Status

Synapse is in early active development. The core `Brain` (Cortex + Memory), the Copilot provider, and the module skeleton are in place. Provider abstraction, the planning loop, database connectors, and the skills system are being built out now.

Contributions, feedback, and use-case ideas are welcome — open an issue or a PR.

---

## License

MIT
