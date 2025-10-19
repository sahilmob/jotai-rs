# Jotai-RS

A Rust implementation of [Jotai](https://github.com/pmndrs/jotai) for **AI ASSISTED** learning purposes. This project rebuilds Jotai's core state management primitives in Rust to deeply understand its internal architecture.

## 🎯 Learning Objectives

- Understand atomic state management principles
- Learn dependency tracking and invalidation strategies
- Master observer pattern and subscription systems
- Explore functional programming patterns in state management
- Study efficient caching and memoization techniques

## 📚 Documentation

See **[CLAUDE.md](./CLAUDE.md)** for comprehensive documentation including:

- Complete architecture analysis of Jotai
- 15+ functional programming patterns with examples
- Detailed 9-phase implementation plan
- Core algorithms (dependency tracking, topological sort, etc.)
- Testing strategy
- Performance considerations

## 🏗️ Project Structure

```
jotai-rs/
├── CLAUDE.md                 # Complete project documentation
├── README.md                 # This file
├── Cargo.toml               # Rust dependencies
├── jotai/                   # Git submodule - reference implementation
├── src/
│   ├── lib.rs              # Main entry point
│   ├── types.rs            # Core type definitions
│   ├── error.rs            # Error types
│   ├── atom.rs             # Atom primitives
│   ├── store.rs            # Store implementation
│   ├── internals.rs        # Internal state structures
│   └── utils/              # Utility atoms
│       ├── atom_family.rs
│       └── select_atom.rs
└── tests/
    ├── basic_atoms.rs      # Phase 1 tests
    └── derived_atoms.rs    # Phase 2 tests
```

## 🚀 Current Status

**Phase**: Setup Complete - Ready for Implementation

**What's Done**:

- ✅ Complete project structure
- ✅ All modules with comprehensive TODO comments
- ✅ Detailed architecture analysis from Jotai source
- ✅ Functional programming patterns documented
- ✅ Test suite scaffolded
- ✅ 9-phase implementation roadmap

**What's Next** (Phase 1):

- Implement atom creation and ID generation
- Implement basic `Store::get()` for primitive atoms
- Implement basic `Store::set()` for primitive atoms
- Pass Phase 1 tests in `tests/basic_atoms.rs`

## 🔧 Building

```bash
# Check compilation (will show expected errors until Phase 1 is implemented)
cargo check

# Run tests (all tests are #[ignore]d until implemented)
cargo test

# Run specific phase tests
cargo test --test basic_atoms
cargo test --test derived_atoms
```

## 📖 Reference Implementation

This project uses the official Jotai TypeScript implementation as reference:

- **Submodule**: `jotai/` (from https://github.com/pmndrs/jotai)
- **Key Files**:
  - `jotai/src/vanilla/atom.ts` - Atom definition
  - `jotai/src/vanilla/store.ts` - Store API
  - `jotai/src/vanilla/internals.ts` - Core engine (1000+ lines)

## 🧩 Core Concepts

### Atoms

Atoms are immutable configuration objects that represent pieces of state:

```rust
// Primitive atom (holds a value)
let count = atom(0);

// Derived atom (computed from other atoms)
let double = atom_derived(|get| {
    let c = get(&count)?;
    Ok(c * 2)
});
```

### Store

The Store manages all atom values and coordinates updates:

```rust
let store = Store::new();

// Read
let value = store.get(&count)?;

// Write
store.set(&count, 42)?;

// Subscribe
let unsub = store.sub(&count, || {
    println!("Count changed!");
});
```

## 🎓 Functional Programming Patterns

This project demonstrates 15+ FP patterns from Jotai:

1. **Higher-Order Functions** - Functions that return functions
2. **Closures** - Captured state in returned functions
3. **Function Composition** - Atoms compose through dependencies
4. **Immutability** - Atoms are immutable configs
5. **Pure Functions** - Deterministic read functions
6. **Lazy Evaluation** - Compute on demand
7. **Memoization** - Cache with epoch numbers
8. **Monadic Patterns** - Reader/State monad-like Getter/Setter
9. **Observer Pattern** - Subscription system
10. **Algebraic Data Types** - Rust enums for variants

And more! See [CLAUDE.md](./CLAUDE.md) for detailed explanations and examples.

## 🗓️ Implementation Phases

| Phase | Week | Focus                 | Key Features                     |
| ----- | ---- | --------------------- | -------------------------------- |
| 1     | 1    | Core Primitives       | Atom creation, basic get/set     |
| 2     | 2    | Dependency Tracking   | Derived atoms, invalidation      |
| 3     | 3    | Subscriptions         | Listeners, mount/unmount         |
| 4     | 4    | Advanced Invalidation | Topological sort                 |
| 5     | 5    | Writable Derived      | Custom write functions           |
| 6     | 6    | Async Support         | Promises, AbortSignal            |
| 7     | 7    | Utility Atoms         | atomFamily, selectAtom           |
| 8     | 8    | Advanced Features     | Lifecycle, error handling        |
| 9-10  | 9-10 | Optimization          | Performance, comprehensive tests |

## 🔑 Key Algorithms

### Dependency Tracking

Automatically tracks which atoms depend on which others during reads.

### Epoch-Based Caching

Uses version numbers instead of deep equality for efficient cache invalidation.

### Topological Sort

Recomputes atoms in correct dependency order using DFS.

### Cascading Updates

Propagates changes through dependency graph until stable.

## 📝 TODO Comments

Every file contains detailed TODO comments linked to implementation phases:

```rust
/// TODO: Phase 1.3 - Implement basic get logic
/// TODO: Phase 2.1 - Add dependency tracking
/// TODO: Phase 4.2 - Add topological sort
```

Use your editor's TODO search to find next steps!

## 🧪 Testing Strategy

Tests are organized by phase and marked with `#[ignore]`:

```rust
#[test]
#[ignore = "Phase 1.3 - Implement store.get()"]
fn test_read_primitive_atom() {
    // Test will pass once Phase 1.3 is complete
}
```

Remove `#[ignore]` attributes as you implement each phase.

## 📚 Learning Resources

### Primary Reference

- Jotai source in `jotai/` submodule
- Focus on `jotai/src/vanilla/` directory

### Rust Concepts

- Trait objects and dynamic dispatch
- `Arc`, `Mutex`, and thread safety
- Type erasure with `Any`
- Closure captures and lifetimes

### State Management

- Atomic state updates
- Dependency graphs
- Reactive programming
- Observer pattern

## 🤝 Contributing to Learning

This is a learning project! If you're also learning Jotai or Rust:

1. Fork the project
2. Implement a phase
3. Share insights and learnings
4. Compare different implementation approaches

## 📄 License

MIT License - This is an educational project for learning purposes.

## 🙏 Acknowledgments

- **Jotai Team** - For the excellent original implementation
- **TypeScript Source** - For being well-documented and readable
- **Rust Community** - For great learning resources

## 🔗 Links

- [Jotai Official Docs](https://jotai.org/)
- [Jotai GitHub](https://github.com/pmndrs/jotai)
- [The Rust Book](https://doc.rust-lang.org/book/)

---

**Start your learning journey**: Read `CLAUDE.md` then jump into `src/atom.rs`!
