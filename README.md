# 🦠 slime

`slime` is a decentralized LLM agent framework inspired by slime mold behavior.
The core unit is a **Mass** that continuously handles fresh I/O through stateless pulses.

![alt text](image.png)

---

## 🧬 Core Principle: Stateless Pulses

This framework focuses on **statelessness**:

- Every pulse is a fresh I/O cycle for LLM calls.
- A pulse does not carry conversational state internally.
- The shared **Blackboard** stores evolving state from user input and pulse output.
- Future pulses read the current blackboard snapshot, then produce new output.

In short: **state lives in memory, not in pulses**.

---

## 🏗️ Refined Structure

```text
slime/
├── Cargo.toml
└── src/
    ├── main.rs                # Spark: create one or more Mass objects
    ├── mass/                  # Protoplasm: decentralized working unit
    │   ├── mod.rs             # Construct logic for Mass
    │   ├── pulse.rs           # Stateless pulse definition
    │   ├── boundary.rs        # Input ingestion + output synthesis
    │   └── memory/
    │       ├── mod.rs
    │       └── blackboard.rs  # Shared thread-safe state container
    ├── skills/
    ├── tools/
    └── runtime/
```

---

## 🧩 Mass Model

- A `Mass` must have:
  - exactly one memory (`Blackboard` via `mass/memory`)
  - one or more pulses (`mass/pulse`)
- A pulse can connect to:
  - `runtime`
  - `skills`
  - `tools`

Current implementation is intentionally placeholder-first to lock architecture before behavior.

---

## 🔁 Pulse Cycle (Conceptual)

1. User input and prior outputs are written into the blackboard.
2. A stateless pulse reads the latest blackboard state.
3. Pulse performs a fresh LLM I/O call.
4. Boundary normalizes/synthesizes output.
5. Blackboard updates with new state.

This repeats with no hidden per-pulse context.
