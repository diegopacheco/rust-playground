# Agent Projects in rust-playground

This repository contains several agent/actor-based projects that demonstrate different actor model frameworks in Rust.

## Overview

The actor model is a concurrency pattern where "actors" are independent units of computation that communicate through message passing. These projects showcase different Rust implementations of this pattern.

## Agent/Actor Projects

### 1. ractor-fun
**Location:** `/ractor-fun`

**Framework:** [Ractor](https://crates.io/crates/ractor) v0.9.0

**Description:** A modern async actor framework built on Tokio. This example demonstrates:
- Actor lifecycle management with `pre_start`
- Message handling with pattern matching
- RPC-style communication with reply ports
- Actor state management

**Key Features:**
- Async/await support via async-trait
- Type-safe message passing
- Request-reply pattern implementation
- Actor spawning and lifecycle management

**Example Usage:**
```rust
// Creates an actor that counts "Hello World" messages
// Demonstrates both one-way (cast) and two-way (call) messaging
```

---

### 2. riker-fun
**Location:** `/riker-fun`

**Framework:** [Riker](https://crates.io/crates/riker) v0.4.0

**Description:** An actor framework inspired by Akka. This example shows:
- Simple actor creation and registration
- Message sending with `tell`
- Actor system initialization

**Key Features:**
- Actor system management
- Location transparency
- Hierarchical actor structure
- Message-driven communication

**Example Usage:**
```rust
// Creates an actor system and sends string messages to actors
```

---

### 3. xactor_fun
**Location:** `/xactor_fun`

**Framework:** [xActor](https://crates.io/crates/xactor) v0.2.3

**Description:** A lightweight async actor framework. This example demonstrates:
- Message handler implementation
- Async message processing
- Actor address-based communication

**Key Features:**
- Built on async-std
- Type-safe message handling
- Simple actor API
- Request-response patterns

**Example Usage:**
```rust
// Creates an actor that processes string transformation messages
```

---

### 4. bastion-fun
**Location:** `/bastion-fun`

**Framework:** [Bastion](https://crates.io/crates/bastion) v0.3.4

**Description:** A fault-tolerant runtime for Rust applications. This example shows:
- Supervisor hierarchies
- Fault tolerance with automatic restarts
- Message pattern matching
- Ask-answer communication patterns

**Key Features:**
- Erlang-inspired fault tolerance
- Automatic actor restart on panic
- Supervisor trees
- Message broadcasting and asking

**Example Usage:**
```rust
// Creates a fault-tolerant child that demonstrates:
// - Message receiving
// - Anonymous asking
// - Automatic restart on panic
```

---

## Comparison

| Framework | Async Runtime | Key Strength | Use Case |
|-----------|--------------|--------------|----------|
| **Ractor** | Tokio | Modern API, strong typing | New projects needing async actors |
| **Riker** | Native threads | Akka-like API | JVM developers familiar with Akka |
| **xActor** | async-std | Lightweight, simple | Simple actor needs |
| **Bastion** | Native | Fault tolerance | Critical systems needing resilience |

## Getting Started

Each project can be built and run independently:

```bash
cd ractor-fun  # or riker-fun, xactor_fun, bastion-fun
cargo build
cargo run
```

## Additional Resources

- [Actor Model on Wikipedia](https://en.wikipedia.org/wiki/Actor_model)
- [Actors in Rust: A Comparison](https://ryhl.io/blog/actors-with-tokio/)
- All these frameworks provide message-based concurrency and isolation

## Related Projects in Repository

While not strictly "agent" projects, these are related concurrent programming examples:
- `channels-fun-rust` - Channel-based communication
- `thread-channel-fun` - Thread and channel examples
- `crossbeam-fun` - Lock-free concurrent programming
- `futures-fun` - Future-based async programming
