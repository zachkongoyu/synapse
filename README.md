# 🦠

A decentralized, biological approach to Large Language Model agents, modeled after the life cycle of slime molds (*Physarum polycephalum*). This project moves away from human-centric "brain" architectures toward a **Reaction-Diffusion** model of intelligence.

---

## 🧬 Philosophy: The Slime Flow

In a standard agentic framework, the entity is a vertebrate: it has a centralized brain, waits for commands, and follows a linear sequence. 

In **Plasmodium LLM**, the organism is a single, polynucleated cell. It does not "think" then "act"—it **pulses**. Intelligence emerges from the rhythmic flow of protoplasm (tokens) toward nutrients (stimuli) across a decentralized membrane.

### Comparison: Vertebrate vs. Plasmodial


| Feature | Standard Agentic Flow (Vertebrate) | Slime Flow (Plasmodial) |
| :--- | :--- | :--- |
| **Trigger** | **Event-Driven:** Awakens only when called. | **Oscillatory:** Always pulsing; intensity varies. |
| **Logic** | **Linear:** Perceive → Plan → Act → Sleep. | **Gradient-Following:** Contraction → Expansion. |
| **Memory** | **Database:** A static "Hippocampus" vault. | **Slime Trails:** Persistent traces in the environment. |
| **Failure** | **Binary:** Error/Timeout stops the loop. | **Atrophy:** Pulse slows down; organism shrinks. |
| **Structure** | **Centralized:** "Cortex" and "Brain" modules. | **Distributed:** "Membrane" and "Flux" modules. |

---

## 🌀 Architectural Iteration

### The Standard Iteration (Linear)
*   **User:** "Track the price of SOL and notify me at $200."  
*   **Agent:** `[Awaken]` -> `[Call API]` -> `[Report Status]` -> `[Die/Sleep]`.  
*   *The agent is a tool that requires constant external energy to function.*

### The Slime Iteration (Oscillatory)
*   **User:** *Drops "SOL $200" nutrient into the environment.*  
*   **The Membrane:** Detects a nutrient spike. The **Pulse** accelerates from 60s to 2s.  
*   **The Flux:** The LLM "flows" toward the nutrient. It leaves a **Trace** (Memory) stating it has checked the price.  
*   **Continued Life:** Even if the user leaves, the pulse continues. The **Trace** acts as a physical attractor, pulling the next pulse back to the market API until the goal is met.

---

## 🏗️ The Anatomy of the Organism

- **`src/vitality/` (The Pulse):** Replaces `breath_of_life`. Defines the `Oscillate` trait that drives the rhythm of contraction (sensing) and expansion (acting).
- **`src/organism/membrane/` (Perception):** Asynchronous sensory points that update a global **Gradient Map** rather than sending direct commands.
- **`src/organism/flux/` (The Core):** The protoplasmic flow. This is where the LLM resides, deciding how the organism’s mass should shift based on current chemical gradients.
- **`src/organism/trace/` (Memory):** Extracellular trails. Embeddings and logs are "secreted" into the environment, influencing future pulses through attraction or repulsion.
- **`src/organism/pseudopodia/` (Interaction):** "False feet." Temporary extensions (Tools/Skills) created to reach specific environmental nutrients (APIs).

---

## ⚙️ Technical Implementation: The Gradient Map

The organism interacts with a shared thread-safe map where concentrations of `Nutrient`, `Repellent`, and `Trace` determine the behavior:

```rust
// The core rhythm of the organism
async fn pulse(&mut self) {
    loop {
        // Sense environment & aggregate signals
        let gradient = self.membrane.contract().await; 
        
        // Adjust metabolic rate based on signal intensity
        self.adapt_metabolism(&gradient);

        // Flow toward stimulus (LLM invocation)
        self.flux.expand(gradient).await;              
        
        // Rhythmic rest
        tokio::time::sleep(self.metabolism).await;     
    }
}