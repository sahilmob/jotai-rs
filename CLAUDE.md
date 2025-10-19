# Jotai-RS: Learning Jotai Internals Through Rust

## Overview

This project is a Rust implementation of [Jotai](https://github.com/pmndrs/jotai), a primitive and flexible state management library for React. The goal is to deeply understand Jotai's internal architecture by rebuilding its core concepts in Rust, focusing on the fundamental state management primitives without the React-specific parts.

**Reference Implementation**: `jotai/` submodule (TypeScript/JavaScript)

## Learning Objectives

1. Understand atomic state management principles
2. Learn dependency tracking and invalidation strategies
3. Master observer pattern and subscription systems
4. Explore functional programming patterns in state management
5. Study efficient caching and memoization techniques
6. Understand epoch-based invalidation
7. Learn topological sorting for dependency graphs

---

## Functional Programming Patterns in Jotai

Jotai is built on strong functional programming principles. Understanding these patterns is crucial for the Rust implementation.

### 1. **Higher-Order Functions (HOFs)**

Functions that accept other functions as parameters or return functions as results.

**Jotai Example**:
```typescript
// atomFamily returns a function that creates atoms
export function atomFamily<Param, AtomType>(
  initializeAtom: (param: Param) => AtomType,  // HOF parameter
  areEqual?: (a: Param, b: Param) => boolean,
): (param: Param) => AtomType  // Returns a function
```

**Rust Equivalent Pattern**:
```rust
pub fn atom_family<P, A, F>(
    initialize_atom: F,
) -> impl Fn(P) -> A
where
    F: Fn(P) -> A,
{
    // Returns a closure
}
```

### 2. **Closures and Lexical Scoping**

Jotai extensively uses closures to capture and encapsulate state.

**Jotai Example**:
```typescript
export function atomFamily(initializeAtom, areEqual) {
  const atoms: Map<Param, AtomType> = new Map()  // Captured in closure

  const createAtom = (param) => {
    // Closure has access to 'atoms' and 'initializeAtom'
    let item = atoms.get(param)
    if (!item) {
      item = initializeAtom(param)
      atoms.set(param, item)
    }
    return item
  }

  createAtom.remove = (param) => atoms.delete(param)  // Also captures 'atoms'
  return createAtom
}
```

**Rust Equivalent Pattern**:
```rust
// Use Arc<Mutex<>> or DashMap for shared mutable state in closures
use std::sync::Arc;
use dashmap::DashMap;

pub fn atom_family<P, A, F>(initialize_atom: F) -> /* ... */
where
    F: Fn(P) -> A,
{
    let atoms = Arc::new(DashMap::new());

    let atoms_clone = atoms.clone();
    move |param: P| {
        // Closure captures atoms_clone
        atoms_clone.entry(param)
            .or_insert_with(|| initialize_atom(param))
            .clone()
    }
}
```

### 3. **Function Composition**

Atoms compose through dependency relationships via `get()` calls.

**Jotai Example**:
```typescript
const firstNameAtom = atom('John')
const lastNameAtom = atom('Doe')
const fullNameAtom = atom((get) => {
  // Composing two atoms through function composition
  return `${get(firstNameAtom)} ${get(lastNameAtom)}`
})
```

**Rust Equivalent Pattern**:
```rust
let first_name = atom("John");
let last_name = atom("Doe");
let full_name = atom(|get: &Getter| {
    format!("{} {}", get(&first_name), get(&last_name))
});
```

### 4. **Immutability**

Atoms are immutable configuration objects. State changes create new values, not mutate existing ones.

**Jotai Example**:
```typescript
// Atom config is created once and never changes
const config = {
  read: (get) => /* ... */,
  write: (get, set, arg) => /* ... */,
  toString: () => key,
}
```

**Rust Equivalent Pattern**:
```rust
pub struct Atom<T> {
    id: AtomId,
    read: Arc<dyn Fn(&Getter) -> T>,
    write: Option<Arc<dyn Fn(&Getter, &Setter, T)>>,
}

// Atom itself is immutable, only state values change
```

### 5. **Pure Functions**

Read functions should be deterministic - same inputs produce same outputs.

**Jotai Example**:
```typescript
// Pure: given same dependencies, always returns same result
const doubleAtom = atom((get) => get(countAtom) * 2)

// Impure (discouraged): depends on external mutable state
let external = 0
const impureAtom = atom((get) => get(countAtom) + external)
```

**Rust Pattern**:
```rust
// Pure function - deterministic
let double = atom(|get: &Getter| get(&count) * 2);

// Should avoid capturing mutable external state
```

### 6. **Lazy Evaluation**

Atoms compute values on-demand, not eagerly.

**Jotai Example**:
```typescript
const expensiveAtom = atom((get) => {
  console.log('Computing...')  // Only logs when atom is read
  return expensiveComputation(get(sourceAtom))
})
```

**Rust Pattern**:
```rust
// Use lazy evaluation through closures
let expensive = atom(|get: &Getter| {
    // Only executed when someone calls store.get(&expensive)
    expensive_computation(get(&source))
});
```

### 7. **Memoization**

Jotai uses WeakMap-based memoization to cache derived atom creation.

**Jotai Example**:
```typescript
const getCached = <T>(c: () => T, m: WeakMap<object, T>, k: object): T =>
  (m.has(k) ? m : m.set(k, c())).get(k) as T

const memo3 = <T>(create: () => T, dep1, dep2, dep3): T => {
  const cache2 = getCached(() => new WeakMap(), cache1, dep1)
  const cache3 = getCached(() => new WeakMap(), cache2, dep2)
  return getCached(create, cache3, dep3)
}
```

**Rust Pattern**:
```rust
use std::collections::HashMap;
use std::sync::Mutex;

// Manual memoization with multiple keys
lazy_static! {
    static ref CACHE: Mutex<HashMap<(usize, usize, usize), AtomResult>>
        = Mutex::new(HashMap::new());
}
```

### 8. **Currying and Partial Application**

Functions return configured functions with some parameters pre-applied.

**Jotai Example**:
```typescript
// selectAtom is a curried function
export function selectAtom<Value, Slice>(
  anAtom: Atom<Value>,
  selector: (v: Value) => Slice,
  equalityFn = Object.is,
) {
  // Returns a new atom with anAtom, selector, equalityFn captured
  return atom((get) => {
    const value = get(anAtom)
    return selector(value)
  })
}
```

**Rust Pattern**:
```rust
pub fn select_atom<V, S, F>(
    source: Atom<V>,
    selector: F,
) -> Atom<S>
where
    F: Fn(V) -> S + 'static,
{
    atom(move |get: &Getter| {
        let value = get(&source);
        selector(value)
    })
}
```

### 9. **First-Class Functions**

Functions are values that can be stored, passed, and returned.

**Jotai Example**:
```typescript
interface Atom<Value> {
  read: Read<Value>  // Function stored as data
}

interface WritableAtom<Value, Args, Result> extends Atom<Value> {
  read: Read<Value>
  write: Write<Args, Result>  // Function stored as data
}
```

**Rust Pattern**:
```rust
pub struct Atom<T> {
    read: Arc<dyn Fn(&Getter) -> T>,  // Trait object for function
    write: Option<Arc<dyn Fn(&Getter, &Setter, T)>>,
}
```

### 10. **Algebraic Data Types (ADTs)**

Using type systems to represent different atom variants.

**Jotai Example**:
```typescript
// Discriminated union through overloading
function atom<Value>(read: Read<Value>): Atom<Value>
function atom<Value>(initialValue: Value): PrimitiveAtom<Value>
function atom<Value, Args, Result>(
  read: Read<Value>,
  write: Write<Args, Result>,
): WritableAtom<Value, Args, Result>
```

**Rust Pattern**:
```rust
pub enum AtomType<T> {
    Primitive { initial: T },
    ReadOnly { read: Arc<dyn Fn(&Getter) -> T> },
    Writable {
        read: Arc<dyn Fn(&Getter) -> T>,
        write: Arc<dyn Fn(&Getter, &Setter, T)>,
    },
}
```

### 11. **Monadic Patterns**

The `get`/`set` pattern resembles Reader and State monads.

**Jotai Example**:
```typescript
// Reader monad pattern - 'get' provides reading context
type Read<Value> = (get: Getter) => Value

// State monad pattern - 'get' reads, 'set' writes
type Write<Args, Result> = (get: Getter, set: Setter, ...args) => Result
```

**Conceptual Mapping**:
- `Getter` ≈ Reader monad environment
- `Setter` ≈ State monad's put operation
- Atom read function ≈ Reader computation
- Atom write function ≈ State transformation

### 12. **Observer Pattern**

Pub/sub system for reactive updates.

**Jotai Example**:
```typescript
// Subscribe to changes
const unsubscribe = store.sub(atom, () => {
  console.log('Atom changed!')
})

// Later unsubscribe
unsubscribe()
```

**Rust Pattern**:
```rust
let unsub = store.subscribe(&atom, Box::new(|| {
    println!("Atom changed!");
}));

// Later
unsub();
```

### 13. **Builder Pattern**

Atom configuration uses builder-like pattern.

**Jotai Example**:
```typescript
const config = {
  toString: () => key,
  read: readFn,
  write: writeFn,
  debugLabel: 'myAtom',
  onMount: (set) => cleanup,
}
```

**Rust Pattern**:
```rust
Atom::builder()
    .id(atom_id)
    .read(read_fn)
    .write(write_fn)
    .debug_label("myAtom")
    .on_mount(mount_fn)
    .build()
```

### 14. **Factory Pattern**

`atom()` and `atomFamily()` are factory functions.

**Jotai Example**:
```typescript
// Factory creates configured objects
const myAtom = atom(0)  // Factory call
const familyFn = atomFamily((id) => atom(id))  // Factory of factories
```

### 15. **Signals and Cancellation (Functional Reactive Programming)**

AbortSignal for cancelling async operations.

**Jotai Example**:
```typescript
type Read<Value> = (
  get: Getter,
  options: { signal: AbortSignal }
) => Value | Promise<Value>

const asyncAtom = atom(async (get, { signal }) => {
  const response = await fetch('/api', { signal })
  return response.json()
})
```

**Rust Pattern**:
```rust
use tokio::sync::watch;

let async_atom = atom(|get: &Getter, signal: &CancellationToken| async move {
    let response = fetch_with_cancel("/api", signal).await?;
    response.json().await
});
```

---

## Implementation Plan

### Phase 1: Core Primitives (Week 1)

**Goal**: Implement basic atom creation and simple store operations.

#### Step 1.1: Atom Definition
- [ ] Define `AtomId` type (unique identifier)
- [ ] Define `Atom<T>` struct with read/write functions
- [ ] Implement `atom()` factory for primitive atoms
- [ ] Implement `atom()` factory for read-only derived atoms
- [ ] Add debug labels and toString

**FP Patterns**: First-class functions, immutability, factory pattern

**Reference**: `jotai/src/vanilla/atom.ts`

#### Step 1.2: Basic Store Structure
- [ ] Define `AtomState<T>` struct (value, dependencies, epoch)
- [ ] Define `Store` struct with `DashMap` for atom states
- [ ] Implement global atom ID counter
- [ ] Implement `Store::new()` constructor

**FP Patterns**: Closures, lazy evaluation

**Reference**: `jotai/src/vanilla/store.ts`, `jotai/src/vanilla/internals.ts:1-100`

#### Step 1.3: Store Get Operation
- [ ] Implement `store.get(atom)` for primitive atoms
- [ ] Implement lazy evaluation (compute on first access)
- [ ] Add simple caching (no dependency tracking yet)

**FP Patterns**: Lazy evaluation, memoization

**Reference**: `jotai/src/vanilla/internals.ts` (readAtomState function)

#### Step 1.4: Store Set Operation
- [ ] Implement `store.set(atom, value)` for primitive atoms
- [ ] Update atom state
- [ ] Increment epoch number

**FP Patterns**: Pure functions for state transitions

**Reference**: `jotai/src/vanilla/internals.ts` (writeAtomState function)

### Phase 2: Dependency Tracking (Week 2)

**Goal**: Implement dependency tracking between atoms.

#### Step 2.1: Dependency Registration
- [ ] Track dependencies during `get()` calls
- [ ] Store dependencies in `AtomState.dependencies`
- [ ] Store reverse dependencies (dependents)

**FP Patterns**: Function composition

**Reference**: `jotai/src/vanilla/internals.ts` (dependency tracking in readAtomState)

#### Step 2.2: Derived Atoms
- [ ] Support read-only derived atoms
- [ ] Implement automatic dependency tracking
- [ ] Test composition of multiple atoms

**FP Patterns**: Function composition, pure functions

**Reference**: `jotai/src/vanilla/atom.ts:82` (read-only atom overload)

#### Step 2.3: Invalidation System
- [ ] Implement dependent invalidation on set
- [ ] Mark all transitive dependents as stale
- [ ] Track invalidated atoms

**FP Patterns**: Recursion for graph traversal

**Reference**: `jotai/src/vanilla/internals.ts` (invalidateDependents function)

#### Step 2.4: Epoch-Based Caching
- [ ] Check dependency epochs before recomputing
- [ ] Skip recomputation if dependencies unchanged
- [ ] Optimize for minimal work

**FP Patterns**: Memoization, lazy evaluation

**Reference**: `jotai/src/vanilla/internals.ts` (epoch checking logic)

### Phase 3: Subscription System (Week 3)

**Goal**: Implement reactive subscriptions and listeners.

#### Step 3.1: Mounted State
- [ ] Define `Mounted` struct (listeners, dependencies, dependents)
- [ ] Track mounted atoms separately
- [ ] Implement mount/unmount lifecycle

**FP Patterns**: Observer pattern, closures

**Reference**: `jotai/src/vanilla/internals.ts` (Mounted type and mountAtom function)

#### Step 3.2: Subscribe Operation
- [ ] Implement `store.sub(atom, listener)`
- [ ] Mount atom on first subscription
- [ ] Track listeners in `Mounted`
- [ ] Return unsubscribe function

**FP Patterns**: Observer pattern, higher-order functions (returns unsubscribe fn)

**Reference**: `jotai/src/vanilla/internals.ts` (storeSub function)

#### Step 3.3: Listener Notification
- [ ] Collect changed atoms after updates
- [ ] Notify all listeners for changed atoms
- [ ] Implement callback flushing loop

**FP Patterns**: Observer pattern, recursion for cascading updates

**Reference**: `jotai/src/vanilla/internals.ts` (flushCallbacks function)

#### Step 3.4: Recursive Mounting
- [ ] Mount dependencies when atom is mounted
- [ ] Unmount when no listeners remain
- [ ] Handle cleanup functions

**FP Patterns**: Recursion, closures for cleanup

**Reference**: `jotai/src/vanilla/internals.ts` (mountDependencies, unmountAtom)

### Phase 4: Advanced Invalidation (Week 4)

**Goal**: Implement topological sorting for correct update order.

#### Step 4.1: Topological Sort Implementation
- [ ] Implement DFS-based topological sort
- [ ] Sort invalidated atoms by dependency order
- [ ] Handle cycles (error or detection)

**FP Patterns**: Recursion, graph algorithms

**Reference**: `jotai/src/vanilla/internals.ts` (recomputeInvalidatedAtoms with DFS)

#### Step 4.2: Recomputation Loop
- [ ] Recompute atoms in topological order
- [ ] Track which atoms actually changed values
- [ ] Short-circuit unchanged atoms

**FP Patterns**: Pure functions, memoization

**Reference**: `jotai/src/vanilla/internals.ts` (recomputeInvalidatedAtoms function)

#### Step 4.3: Cascading Updates
- [ ] Handle updates that trigger more updates
- [ ] Loop until stable (no more changes)
- [ ] Prevent infinite loops

**FP Patterns**: Recursion, fixpoint computation

**Reference**: `jotai/src/vanilla/internals.ts` (flushCallbacks loop)

### Phase 5: Writable Derived Atoms (Week 5)

**Goal**: Support atoms with custom write functions.

#### Step 5.1: Write Function Support
- [ ] Store write function in atom config
- [ ] Implement write function signature (get, set, args)
- [ ] Allow updating other atoms from write

**FP Patterns**: Higher-order functions, function composition

**Reference**: `jotai/src/vanilla/atom.ts:76-79` (writable derived atom)

#### Step 5.2: SetAtom Type
- [ ] Implement `SetAtom<Args, Result>` type
- [ ] Pass to read function for self-updates
- [ ] Support async patterns

**FP Patterns**: Closures, callbacks

**Reference**: `jotai/src/vanilla/atom.ts:10-12` (SetAtom type)

#### Step 5.3: Complex Update Patterns
- [ ] Test atom updating multiple other atoms
- [ ] Test conditional updates
- [ ] Test reducer-like patterns

**FP Patterns**: Function composition, state transformations

**Reference**: `jotai/src/vanilla/utils/atomWithReducer.ts`

### Phase 6: Async Support (Week 6)

**Goal**: Handle promises and async operations.

#### Step 6.1: Promise State Tracking
- [ ] Detect when read returns Promise
- [ ] Store promise in AtomState
- [ ] Track pending promises

**FP Patterns**: Monadic patterns (Promise is a monad)

**Reference**: `jotai/src/vanilla/internals.ts` (Promise handling in setAtomStateValueOrPromise)

#### Step 6.2: AbortSignal Integration
- [ ] Pass AbortSignal to read function
- [ ] Cancel pending promises on invalidation
- [ ] Clean up on unmount

**FP Patterns**: Signals (functional reactive programming)

**Reference**: `jotai/src/vanilla/internals.ts` (AbortController usage)

#### Step 6.3: Promise Resolution Handling
- [ ] Recompute dependents when promise settles
- [ ] Handle errors in async atoms
- [ ] Support cascading async updates

**FP Patterns**: Async/await patterns, error handling

**Reference**: `jotai/src/vanilla/internals.ts` (promise .then() handlers)

### Phase 7: Utility Atoms (Week 7)

**Goal**: Implement common utility atom factories.

#### Step 7.1: Atom Family
- [ ] Implement `atomFamily(initFn, equalFn)`
- [ ] Memoize atom creation by parameter
- [ ] Support custom equality
- [ ] Implement remove and cleanup

**FP Patterns**: Higher-order functions, closures, memoization, factory pattern

**Reference**: `jotai/src/vanilla/utils/atomFamily.ts`

#### Step 7.2: Select Atom
- [ ] Implement `selectAtom(source, selector, equality)`
- [ ] Memoize selector results
- [ ] Skip updates if selected value unchanged

**FP Patterns**: Function composition, memoization, currying

**Reference**: `jotai/src/vanilla/utils/selectAtom.ts`

#### Step 7.3: Atom with Reducer
- [ ] Implement `atomWithReducer(initial, reducer)`
- [ ] Support dispatch-style updates
- [ ] Test reducer patterns

**FP Patterns**: Reducer pattern, pure functions

**Reference**: `jotai/src/vanilla/utils/atomWithReducer.ts`

#### Step 7.4: Atom with Default
- [ ] Implement `atomWithDefault(read)`
- [ ] Support resettable atoms
- [ ] Handle fallback values

**FP Patterns**: Higher-order functions

**Reference**: `jotai/src/vanilla/utils/atomWithDefault.ts`

### Phase 8: Advanced Features (Week 8)

**Goal**: Implement debugging and monitoring capabilities.

#### Step 8.1: OnMount Lifecycle
- [ ] Support `onMount` callback in atom config
- [ ] Call when atom first mounted
- [ ] Execute cleanup on unmount

**FP Patterns**: Closures for cleanup, callbacks

**Reference**: `jotai/src/vanilla/atom.ts:36-40`, `internals.ts` (mountAtom)

#### Step 8.2: Store Hooks
- [ ] Implement hook system for monitoring
- [ ] Add hooks for read, write, mount, unmount
- [ ] Support dev tools integration

**FP Patterns**: Observer pattern, middleware pattern

**Reference**: `jotai/src/vanilla/internals.ts` (StoreHooks type)

#### Step 8.3: Error Handling
- [ ] Catch errors in read functions
- [ ] Store errors in AtomState
- [ ] Propagate errors to subscribers
- [ ] Support error boundaries

**FP Patterns**: Result/Either types

**Reference**: `jotai/src/vanilla/internals.ts` (error handling in readAtomState)

#### Step 8.4: OnInit Lifecycle
- [ ] Support `unstable_onInit` callback
- [ ] Fire when atom first referenced by store
- [ ] Use for initialization side effects

**FP Patterns**: Callbacks, lazy initialization

**Reference**: `jotai/src/vanilla/atom.ts:55`, `internals.ts` (ensureAtomState)

### Phase 9: Optimization & Testing (Week 9-10)

**Goal**: Optimize performance and add comprehensive tests.

#### Step 9.1: Performance Optimization
- [ ] Benchmark core operations
- [ ] Optimize hot paths
- [ ] Reduce allocations
- [ ] Profile memory usage

#### Step 9.2: Comprehensive Testing
- [ ] Unit tests for each component
- [ ] Integration tests for complex scenarios
- [ ] Property-based tests
- [ ] Stress tests with many atoms

#### Step 9.3: Documentation
- [ ] Document all public APIs
- [ ] Add examples for each pattern
- [ ] Write architecture guide
- [ ] Create migration guide from Jotai

#### Step 9.4: Edge Cases
- [ ] Test circular dependencies (should error)
- [ ] Test memory leaks
- [ ] Test concurrent access
- [ ] Test error propagation

---

## Project Structure

```
jotai-rs/
├── Cargo.toml
├── CLAUDE.md                 # This file - project documentation
├── README.md                 # Public API documentation
├── LICENSE
├── jotai/                    # Git submodule - reference implementation
├── src/
│   ├── lib.rs               # Main entry point
│   ├── atom.rs              # Atom primitives and factory
│   ├── store.rs             # Store implementation
│   ├── internals.rs         # Core state management engine
│   ├── types.rs             # Common types (AtomId, Getter, Setter, etc.)
│   ├── error.rs             # Error types
│   └── utils/               # Utility atoms
│       ├── mod.rs
│       ├── atom_family.rs   # atomFamily implementation
│       ├── select_atom.rs   # selectAtom implementation
│       ├── atom_with_reducer.rs
│       ├── atom_with_default.rs
│       └── loadable.rs
├── tests/
│   ├── basic_atoms.rs       # Basic atom tests
│   ├── derived_atoms.rs     # Dependency tests
│   ├── subscriptions.rs     # Subscription tests
│   ├── async_atoms.rs       # Async/promise tests
│   └── utilities.rs         # Utility atom tests
├── benches/
│   └── store_bench.rs       # Performance benchmarks
└── examples/
    ├── counter.rs           # Simple counter example
    ├── todo_list.rs         # Todo list with atomFamily
    └── async_fetch.rs       # Async data fetching
```

---

## Core Type Definitions

### Rust Types (to be implemented)

```rust
// types.rs
pub type AtomId = usize;
pub type EpochNumber = u64;

pub trait Getter {
    fn get<T: 'static>(&self, atom: &Atom<T>) -> T;
}

pub trait Setter {
    fn set<T: 'static>(&self, atom: &Atom<T>, value: T);
}

// atom.rs
pub struct Atom<T> {
    id: AtomId,
    read_fn: Arc<dyn Fn(&dyn Getter) -> T + Send + Sync>,
    write_fn: Option<Arc<dyn Fn(&dyn Getter, &dyn Setter, T) + Send + Sync>>,
    debug_label: Option<String>,
}

// internals.rs
pub struct AtomState<T> {
    dependencies: HashMap<AtomId, EpochNumber>,
    epoch: EpochNumber,
    value: Option<Result<T, AtomError>>,
}

pub struct Mounted {
    listeners: Vec<Box<dyn Fn() + Send>>,
    dependencies: HashSet<AtomId>,
    dependents: HashSet<AtomId>,
    cleanup: Option<Box<dyn FnOnce()>>,
}

// store.rs
pub struct Store {
    atom_states: DashMap<AtomId, Box<dyn Any>>,  // Type-erased AtomState
    mounted: DashMap<AtomId, Mounted>,
    invalidated: DashMap<AtomId, EpochNumber>,
}

impl Store {
    pub fn get<T: 'static>(&self, atom: &Atom<T>) -> T;
    pub fn set<T: 'static>(&self, atom: &Atom<T>, value: T);
    pub fn sub<F>(&self, atom: &Atom<T>, listener: F) -> impl FnOnce()
    where
        F: Fn() + Send + 'static;
}
```

---

## Key Algorithms

### 1. Dependency Tracking

```rust
// When reading an atom during another atom's computation:
fn track_dependency(&mut self, dependent: AtomId, dependency: AtomId, epoch: EpochNumber) {
    // Store dependency with its epoch number
    self.atom_states.get_mut(&dependent)
        .dependencies
        .insert(dependency, epoch);

    // Store reverse reference
    self.mounted.get_mut(&dependency)
        .dependents
        .insert(dependent);
}
```

**Reference**: `jotai/src/vanilla/internals.ts` lines ~300-350

### 2. Invalidation Algorithm

```rust
fn invalidate_dependents(&mut self, changed_atom: AtomId) {
    let mut to_invalidate = vec![changed_atom];
    let mut invalidated = HashSet::new();

    // Breadth-first traversal
    while let Some(atom_id) = to_invalidate.pop() {
        if invalidated.contains(&atom_id) {
            continue;
        }
        invalidated.insert(atom_id);

        // Mark as invalidated
        self.invalidated.insert(atom_id);

        // Add all dependents to queue
        if let Some(mounted) = self.mounted.get(&atom_id) {
            to_invalidate.extend(&mounted.dependents);
        }
    }
}
```

**Reference**: `jotai/src/vanilla/internals.ts` (invalidateDependents function)

### 3. Topological Sort for Recomputation

```rust
fn recompute_invalidated(&mut self) {
    // Depth-first search for topological ordering
    let sorted = self.topological_sort(self.invalidated.keys());

    let mut changed = HashSet::new();

    // Recompute in dependency order (leaves first)
    for atom_id in sorted {
        let old_value = self.atom_states.get(&atom_id).value.clone();
        let new_value = self.recompute_atom(atom_id);

        if old_value != new_value {
            changed.insert(atom_id);
            self.invalidate_dependents(atom_id);
        }
    }

    self.changed_atoms = changed;
}

fn topological_sort(&self, atoms: impl Iterator<Item = AtomId>) -> Vec<AtomId> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut visiting = HashSet::new();

    for atom_id in atoms {
        self.dfs(atom_id, &mut visited, &mut visiting, &mut result);
    }

    result
}

fn dfs(&self, atom: AtomId, visited: &mut HashSet<AtomId>,
       visiting: &mut HashSet<AtomId>, result: &mut Vec<AtomId>) {
    if visited.contains(&atom) {
        return;
    }
    if visiting.contains(&atom) {
        panic!("Circular dependency detected");
    }

    visiting.insert(atom);

    // Visit dependencies first
    if let Some(state) = self.atom_states.get(&atom) {
        for dep in state.dependencies.keys() {
            self.dfs(*dep, visited, visiting, result);
        }
    }

    visiting.remove(&atom);
    visited.insert(atom);
    result.push(atom);
}
```

**Reference**: `jotai/src/vanilla/internals.ts` (recomputeInvalidatedAtoms with DFS)

### 4. Callback Flushing Loop

```rust
fn flush_callbacks(&mut self) {
    loop {
        // Collect all listeners for changed atoms
        let mut callbacks = Vec::new();
        for atom_id in &self.changed_atoms {
            if let Some(mounted) = self.mounted.get(atom_id) {
                callbacks.extend(mounted.listeners.iter().cloned());
            }
        }

        self.changed_atoms.clear();

        // Execute all callbacks (may cause more changes)
        for callback in callbacks {
            callback();
        }

        // If no new changes, we're done
        if self.changed_atoms.is_empty() &&
           self.mount_callbacks.is_empty() &&
           self.unmount_callbacks.is_empty() {
            break;
        }
    }
}
```

**Reference**: `jotai/src/vanilla/internals.ts` (flushCallbacks function)

---

## Testing Strategy

### Unit Tests

Each phase should have corresponding unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_atom() {
        let store = Store::new();
        let count = atom(0);

        assert_eq!(store.get(&count), 0);
        store.set(&count, 5);
        assert_eq!(store.get(&count), 5);
    }

    #[test]
    fn test_derived_atom() {
        let store = Store::new();
        let count = atom(0);
        let double = atom(|get| get(&count) * 2);

        assert_eq!(store.get(&double), 0);
        store.set(&count, 5);
        assert_eq!(store.get(&double), 10);
    }

    #[test]
    fn test_subscription() {
        let store = Store::new();
        let count = atom(0);

        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let unsub = store.sub(&count, move || {
            called_clone.store(true, Ordering::SeqCst);
        });

        store.set(&count, 1);
        assert!(called.load(Ordering::SeqCst));

        unsub();
    }
}
```

### Integration Tests

Test complex scenarios with multiple atoms:

```rust
#[test]
fn test_complex_dependency_chain() {
    let store = Store::new();

    let a = atom(1);
    let b = atom(|get| get(&a) + 1);
    let c = atom(|get| get(&b) * 2);
    let d = atom(|get| get(&b) + get(&c));

    assert_eq!(store.get(&d), 6);  // (1+1) + ((1+1)*2) = 2 + 4 = 6

    store.set(&a, 2);
    assert_eq!(store.get(&d), 9);  // (2+1) + ((2+1)*2) = 3 + 6 = 9
}
```

---

## Performance Considerations

### Benchmarks to Implement

1. **Get Performance**: Measure `store.get()` with various dependency depths
2. **Set Performance**: Measure `store.set()` with various numbers of dependents
3. **Subscription Overhead**: Measure listener notification cost
4. **Memory Usage**: Track memory consumption with many atoms
5. **Concurrent Access**: Measure contention with multiple threads

### Optimization Strategies

1. **Use DashMap**: Lock-free concurrent hash map for atom states
2. **Arc for Sharing**: Use `Arc` instead of `Rc` for thread-safety
3. **Type Erasure**: Use `Box<dyn Any>` to store heterogeneous atom states
4. **Batch Updates**: Consider batching invalidation and recomputation
5. **Weak References**: Use weak references where appropriate to allow GC

---

## Learning Resources

### Primary Reference
- Jotai source code in `jotai/` submodule
- Focus on `jotai/src/vanilla/` directory
- Key file: `jotai/src/vanilla/internals.ts` (1000+ lines of core logic)

### Functional Programming Concepts
- Higher-order functions and closures
- Immutability and pure functions
- Lazy evaluation and memoization
- Monads (Reader, State, Promise)
- Observer pattern

### Rust Concepts
- Trait objects and dynamic dispatch
- Arc, Mutex, and thread safety
- Type erasure with `Any`
- Closure captures and lifetimes

### State Management Patterns
- Atomic state updates
- Dependency graphs and topological sorting
- Epoch-based invalidation
- Reactive programming

---

## MCP Server Integration

### Jotai MCP Server

An MCP (Model Context Protocol) server will be added to provide AI assistants with access to the Jotai documentation and implementation patterns.

**Location**: `mcp-server/` directory

**Features**:
- Query Jotai TypeScript source code
- Search for specific patterns and implementations
- Get FP pattern examples
- Compare Rust and TypeScript implementations
- Access this CLAUDE.md documentation

**Implementation**:
```bash
# To be implemented in Phase 10
cd mcp-server
npm init -y
# Set up MCP server with tools for code exploration
```

---

## Progress Tracking

### Phase Completion Checklist

- [x] **Phase 1.1**: Atom Definition (atom.rs) ✅ COMPLETE
  - [x] Atom types (Atom, WritableAtom, PrimitiveAtom)
  - [x] Factory functions (atom, atom_derived, atom_writable, atom_write_only)
  - [x] ID generation with AtomicUsize
  - [x] Debug labels and string representation
  - [x] Comprehensive test suite (21 tests)
- [ ] **Phase 1.2**: Basic Store Structure (store.rs)
  - [ ] Store struct with DashMap
  - [ ] Type erasure with Box<dyn Any>
  - [ ] Store::new() constructor
- [ ] **Phase 1.3**: Store Get Operation
  - [ ] Implement Getter trait for Store
  - [ ] Store::get() for primitive atoms
  - [ ] Lazy evaluation
  - [ ] Simple caching (no dependencies yet)
- [ ] **Phase 1.4**: Store Set Operation
  - [ ] Implement Setter trait for Store
  - [ ] Store::set() for primitive atoms
  - [ ] Update atom state
  - [ ] Increment epoch numbers
- [ ] Phase 2: Dependency Tracking (Week 2)
- [ ] Phase 3: Subscription System (Week 3)
- [ ] Phase 4: Advanced Invalidation (Week 4)
- [ ] Phase 5: Writable Derived Atoms (Week 5)
- [ ] Phase 6: Async Support (Week 6)
- [ ] Phase 7: Utility Atoms (Week 7)
- [ ] Phase 8: Advanced Features (Week 8)
- [ ] Phase 9: Optimization & Testing (Week 9-10)
- [ ] Phase 10: MCP Server (Week 10)

### Current Status

**Last Updated**: 2025-10-19 (Session 2)

**Current Phase**: Phase 1.1 - Core Primitives ✅ COMPLETE

**Completed Tasks**:

#### Phase 0 - Project Setup ✅
- [x] Project initialization
- [x] Cargo.toml configuration with dependencies
- [x] Architecture analysis of Jotai (comprehensive)
- [x] FP patterns documentation (15 patterns documented)
- [x] Implementation plan creation (9 phases, 10 weeks)
- [x] Complete module structure created
- [x] All core modules with TODO comments:
  - `lib.rs` - Main entry point with re-exports
  - `types.rs` - Core type definitions (Getter, Setter, AtomId, etc.)
  - `error.rs` - Error types with thiserror
  - `atom.rs` - Atom primitives and factory functions (SKELETON)
  - `store.rs` - Store implementation skeleton
  - `internals.rs` - Internal state structures (AtomState, Mounted)
  - `utils/atom_family.rs` - Atom family utility skeleton
  - `utils/select_atom.rs` - Select atom utility skeleton
- [x] Comprehensive test files:
  - `tests/basic_atoms.rs` - Phase 1 tests (primitive atoms, get/set)
  - `tests/derived_atoms.rs` - Phase 2 tests (dependencies, invalidation)
- [x] This CLAUDE.md documentation file

#### Phase 1.1 - Atom Definition ✅ COMPLETE
- [x] Implemented `AtomId` type with global atomic counter
- [x] Implemented `next_atom_id()` function with atomic increment
- [x] Implemented `Atom<T>` struct with read_fn and debug_label
- [x] Implemented `WritableAtom<T>` struct extending Atom
- [x] Implemented `atom()` factory for primitive atoms (placeholder)
- [x] Implemented `atom_derived()` factory for read-only atoms (placeholder)
- [x] Implemented `atom_writable()` factory for writable derived atoms (placeholder)
- [x] Implemented `atom_write_only()` factory for write-only atoms (placeholder)
- [x] Added `with_label()` builder method for both Atom and WritableAtom
- [x] Implemented `to_string()` and Display trait
- [x] Implemented Debug trait for Atom and WritableAtom
- [x] **FIXED**: Placeholder functions use `Arc::new(|| unreachable!())` pattern
- [x] **FIXED**: Closure ownership issues in `atom_write_only`
- [x] **FIXED**: Unused parameter warnings with `_` prefix
- [x] **COMPREHENSIVE TESTS**: 21 tests covering all atom functionality
  - ID generation (sequential, unique, global counter)
  - Debug labels (setting, formatting, edge cases)
  - String representation (with/without labels)
  - Atom creation (primitive atoms with various types)
  - Cloning and ownership
  - Type safety (complex types, Option, Result)
  - Builder pattern
  - Edge cases (empty labels, unicode, long labels)

**Key Implementation Decisions**:
1. **Option 3 (Placeholder Pattern)**: Used `unreachable!()` for read/write functions in Phase 1, actual logic will be in Store
2. **Type Erasure Workaround**: Deferred dyn-safety issues to Phase 2 when Store is implemented
3. **Closure Captures**: Used `.clone()` inside closures for repeatable execution
4. **Test Strategy**: Disabled derived atom tests until Store is available (Phase 2)

**Compilation Status**: ✅ **Compiles successfully** with only harmless warnings
- 66 unused variable warnings (expected for skeleton code)
- No compilation errors

**Test Status**: ✅ **21/21 tests passing** in `atom::tests`

**Next Steps** (Phase 1.2 - 1.4):
1. ~~Implement atom creation and ID generation~~ ✅ DONE
2. Implement basic Store structure (Phase 1.2)
3. Implement basic Store::get() for primitive atoms (Phase 1.3)
4. Implement basic Store::set() for primitive atoms (Phase 1.4)
5. Run and pass basic_atoms.rs integration tests

---

## Notes and Insights

### Key Insights from Jotai Analysis

1. **Atoms are Cheap**: Creating atoms should be very cheap - they're just config objects
2. **Lazy Everything**: Don't compute until someone asks
3. **Epochs are Powerful**: Version numbers enable smart caching
4. **WeakMaps Enable GC**: Unmounted atoms can be garbage collected
5. **Topological Sort is Critical**: Ensures correct recomputation order
6. **Building Blocks Pattern**: Makes the system extensible
7. **Separate Mounted State**: Only track subscribed atoms for efficiency

### Rust-Specific Challenges

1. **Type Erasure**: Need to handle heterogeneous atom types
2. **Thread Safety**: Jotai is single-threaded (JS), Rust needs Send+Sync
3. **Lifetimes**: Function closures and lifetimes can be tricky
4. **No WeakMap**: Need alternative for garbage collection
5. **Dynamic Dispatch**: Trait objects add some overhead

### Design Decisions

1. **Use DashMap**: Better than `Mutex<HashMap>` for concurrent access
2. **Arc everywhere**: Share data across threads safely
3. **Box<dyn Any>**: Store heterogeneous atom states
4. **No runtime type checking**: Leverage Rust's type system where possible
5. **Explicit lifetimes**: May need to relax some lifetime constraints

---

## Contributing Guidelines

When implementing each phase:

1. **Read the Reference**: Always check the TypeScript implementation first
2. **Write Tests First**: TDD approach recommended
3. **Document FP Patterns**: Note which FP patterns each implementation uses
4. **Add TODO Comments**: Mark areas needing optimization or completion
5. **Benchmark**: Add benchmarks for performance-critical paths
6. **Update This Doc**: Keep CLAUDE.md updated with progress and insights

---

## References

### Jotai Documentation
- [Jotai Official Docs](https://jotai.org/)
- [Jotai GitHub](https://github.com/pmndrs/jotai)
- [Jotai Internals Article](https://blog.axlight.com/posts/how-jotai-works/)

### Functional Programming
- "Functional Programming in JavaScript" - Luis Atencio
- "Programming with Types" - Vlad Riscutia
- Category Theory for Programmers - Bartosz Milewski

### Rust Resources
- The Rust Book - Chapter 13 (Functional Features)
- Rust by Example - Closures and Higher-Order Functions
- "Programming Rust" - Chapter 14 (Closures)

### State Management Theory
- "The Elm Architecture"
- "Reactive Programming Patterns"
- "Observable State Management Patterns"

---

End of CLAUDE.md
