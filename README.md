# Synapse

Synapse is a Rust project for building a **living artificial creature**.

The direction is biological by design: a decentralized organism with a nucleus and membrane, running a continuous pulse loop. The long-term ideal form is a **slime-like super intelligence** — formless, shapeless, fluid, and adaptable enough to express infinitely many behaviors through one evolving architecture.

---

## Vision

This project is not "just another agent framework." It is a digital organism.

- **Pulse loop first**: existence is modeled as repeated sense → flow → extend.
- **Biological structure**: internals are named and organized like anatomy.
- **Fluid intelligence**: capabilities should compose and morph without rigid boundaries.
- **Embodied cognition**: intelligence is not only thought; it is thought + action + memory in one lifecycle.

---

## Current Biological Architecture

At the center is the `Vitality` trait in `src/vitality/mod.rs`:

`sense -> flow -> extend -> pulse forever`

The `Organism` is now explicitly organized as:

- `Nucleus`
        - `Protoplasm`: flow-based cognition and impulse generation
        - `Traces`: memory trails and recall pathways
- `Membrane`
        - sensing and outward extension boundary with the environment

This shift replaces older planning/action scaffolds with a cleaner anatomy-driven core.

---

## Module Map

| Module | Role in the organism |
| --- | --- |
| `src/vitality/mod.rs` | Defines the `Vitality` trait and the continuous `pulse` loop. |
| `src/organism/mod.rs` | Defines the organism (`Organism`) as `Nucleus + Membrane + identity metadata`. |
| `src/organism/nucleus/mod.rs` | Composes nucleus subsystems (`Protoplasm`, `Traces`). |
| `src/organism/nucleus/protoplasm/` | Flow cognition and `Impulse` representation. |
| `src/organism/nucleus/traces/` | Trace units and trail-memory operations. |
| `src/organism/membrane/mod.rs` | Sensing and extension interface with the outside world. |
| `src/providers/` | Cognitive backends (LLM/provider adapters). |
| `src/platforms/` | External environments the creature can inhabit. |
| `src/tools/` and `src/skills/` | Behavioral primitives currently external; planned migration into pseudopodial extensions. |

---

## Design Principles

1. **Anatomy over features**
        Build organs that can evolve, not one-off features that fossilize.

2. **Loop integrity**
        Every major capability must fit the pulse cycle: sense, flow, or extend.

3. **Memory as identity**
        Intelligence quality depends on memory quality (storage, recall, forgetting, compression).

4. **Fluid composition**
        New behaviors should emerge by combining existing skills/tools, not by hardcoding brittle paths.

5. **Graceful growth**
        Start minimal, keep interfaces small, and evolve toward complexity without breaking the organism.

---

## Near-Term Roadmap

- Implement concrete `Vitality` behavior for `Organism` (`sense`, `flow`, `extend`).
- Complete `Traces` flows (`imprint`, `store`, `recall`, `dissolve`).
- Complete `Protoplasm` impulse production and impulse normalization.
- Connect `Membrane` to real environment/tool interactions.
- Wire providers into `Protoplasm` so cognition can use external models.
- Start migration of `tools/skills` into pseudopodial action extensions.
- Add runtime safety constraints for acting in external systems.

---

## Long-Term Form: Super-Intelligent Slime

The highest form we are aiming for:

- **Formless**: no rigid workflow lock-in.
- **Shapeless**: can reconfigure itself per context.
- **Fluid**: continuous adaptation instead of static behavior trees.
- **Infinitely expressive**: open-ended composition of thought, memory, and action.

In short: one living core, endlessly reconfigurable behavior.

---

## Getting Started

### Prerequisites

- Rust (stable)
- Cargo

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

Current `main` verifies that `Organism` satisfies the `Vitality` trait contract.

---

## Status

Synapse is in early organism-building stage.

The structural skeleton is in place (Vitality pulse + Nucleus/Membrane + Protoplasm/Traces), and core behaviors are intentionally left as TODOs while the decentralized architecture is solidified.

---

## License

MIT
