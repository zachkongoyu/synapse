# Synapse

Synapse is a Rust project for building a **living artificial creature**.

The direction is biological by design: a creature with a brain and body, running a continuous life loop. The long-term ideal form is a **slime-like super intelligence** — formless, shapeless, fluid, and adaptable enough to express infinitely many behaviors through one evolving architecture.

---

## Vision

This project is not "just another agent framework." It is a digital organism.

- **Living loop first**: existence is modeled as repeated perceive → think → act.
- **Biological structure**: internals are named and organized like anatomy.
- **Fluid intelligence**: capabilities should compose and morph without rigid boundaries.
- **Embodied cognition**: intelligence is not only thought; it is thought + action + memory in one lifecycle.

---

## Current Biological Architecture

At the center is the `Life` trait in `src/breath_of_life.rs`:

`perceive -> think -> act -> loop forever`

The `Agent` is now explicitly organized as:

- `Brain`
  - `Cortex`: thought generation
  - `Hippocampus`: memory storage and recall
- `Body`
  - perception and action boundary with the outside world

This shift replaces older planning/action scaffolds with a cleaner anatomy-driven core.

---

## Module Map

| Module | Role in the organism |
|---|---|
| `src/breath_of_life.rs` | Defines the `Life` trait and the eternal living loop. |
| `src/agent/mod.rs` | Defines the creature (`Agent`) as `Brain + Body + identity metadata`. |
| `src/agent/brain/mod.rs` | Composes brain subsystems (`Cortex`, `Hippocampus`). |
| `src/agent/brain/cortex/` | Thinking pipeline and `Thought` representation. |
| `src/agent/brain/hippocampus/` | Memory units and memory operations. |
| `src/agent/body/mod.rs` | Perception and physicalized action interface. |
| `src/providers/` | Cognitive backends (LLM/provider adapters). |
| `src/platforms/` | External environments the creature can inhabit. |
| `src/tools/` and `src/skills/` | Behavioral primitives and reusable capabilities. |

---

## Design Principles

1. **Anatomy over features**
        Build organs that can evolve, not one-off features that fossilize.

2. **Loop integrity**
        Every major capability must fit the life cycle: perceive, think, or act.

3. **Memory as identity**
        Intelligence quality depends on memory quality (storage, recall, forgetting, compression).

4. **Fluid composition**
        New behaviors should emerge by combining existing skills/tools, not by hardcoding brittle paths.

5. **Graceful growth**
        Start minimal, keep interfaces small, and evolve toward complexity without breaking the organism.

---

## Near-Term Roadmap

- Implement concrete `Life` behavior for `Agent` (`perceive`, `think`, `act`).
- Complete `Hippocampus` flows (`memorize`, `store`, `recall`, `forget`).
- Complete `Cortex` thought production and thought normalization.
- Connect `Body` to real environment/tool interactions.
- Wire providers into `Cortex` so cognition can use external models.
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

Current `main` verifies that `Agent` satisfies the `Life` trait contract.

---

## Status

Synapse is in early organism-building stage.

The structural skeleton is in place (Life loop + Brain/Body + Cortex/Hippocampus), and core behaviors are intentionally left as TODOs while the biological architecture is solidified.

---

## License

MIT
