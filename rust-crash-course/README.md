# Overview

This project contains small, focused examples of Rust topics. Each topic is a module called from `main`.

## VSCode Plugin for Rust

https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

## Topics

### if / else (`ex_if_else`)

Branch on conditions to control program flow. Use `else if` for multiple mutually exclusive paths.
Conditions must evaluate to `bool`. This example checks parity and divisibility.

```rust
let x = 7;
if x % 2 == 0 { println!("even"); }
else if x % 3 == 0 { println!("divisible by three"); }
else { println!("odd"); }
```

### Option unwrap (`ex_option_unwrap`)

`Option<T>` represents a value that may be present (`Some`) or absent (`None`).
Use `unwrap` when you’re sure it’s `Some`, or `unwrap_or` to provide a default. `map` transforms the inner value.

```rust
let a: Option<i32> = Some(10);
let b: Option<i32> = None;
let x = a.unwrap();
let y = b.unwrap_or(0);
let z = a.map(|v| v * 2).unwrap();
```

### for loops (`ex_for_loops`)

Iterate ranges and collections with `for`. Borrow with `&` to avoid moving values.
Use iterator adapters like `rev()` for reverse iteration, and accumulate with a running variable.

```rust
let mut sum = 0;
for i in 0..5 { sum += i; }
let v = vec![1,2,3];
for x in &v { println!("{}", x); }
for x in v.into_iter().rev() { println!("{}", x); }
```

### traits (`ex_traits`)

Traits define shared behavior; structs implement them. Call trait methods via trait bounds.
Generics with `T: Trait` let functions accept any type implementing the trait.

```rust
trait Greeter { fn greet(&self) -> String; }
struct Person { name: String }
impl Greeter for Person { fn greet(&self) -> String { format!("hello {}", self.name) } }
fn greet_all<T: Greeter>(items: &[T]) { for it in items { println!("{}", it.greet()); } }
```

### HTTP call (`ex_http`)

Use `reqwest` with `tokio` for async HTTP. Build a `Client`, send a request, await the response.
Read status and body; here we only print the status and length for brevity.

```rust
use reqwest::Client;
let c = Client::new();
let res = c.get("https://httpbin.org/get").send().await?;
let status = res.status();
let text = res.text().await?;
```

### JSON serde (`ex_json_serde`)

Derive `Serialize` and `Deserialize` to convert structs to and from JSON.
`serde_json::to_string` and `from_str` do the conversion; keep types consistent.

```rust
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct User { id: u32, name: String, active: bool }
let u = User { id: 1, name: "neo".into(), active: true };
let s = serde_json::to_string(&u).unwrap();
let d: User = serde_json::from_str(&s).unwrap();
```

### Shell command (`ex_shell_cmd`)

Run commands with `std::process::Command` and capture output. Convert bytes to `String` to parse.
Split output by lines to iterate over results or extract fields.

```rust
use std::process::Command;
let out = Command::new("sh").arg("-c").arg("echo rust && uname -s").output()?;
let s = String::from_utf8(out.stdout)?;
let lines: Vec<&str> = s.lines().collect();
```

### Borrow checker (`ex_borrow_checker`)

Multiple immutable borrows allow concurrent reads; a mutable borrow requires exclusive access.
Create mutable borrows only after immutable ones are no longer used.

```rust
fn len(s: &String) -> usize { s.len() }
let mut s = String::from("abc");
let l = len(&s);
let r1 = &s; let r2 = &s;
let r3 = &mut s; r3.push('d');
```

### Unit tests (`ex_unit_tests`)

Place tests in a `#[cfg(test)]` module and use `#[test]` functions with assertions.
Run with `cargo test`; tests are compiled and run separately from the binary.

```rust
pub fn add(a:i32,b:i32)->i32{a+b}
#[cfg(test)]
mod tests{ use super::*; #[test] fn test_add(){ assert_eq!(add(2,3),5); } }
```

### Collections (`ex_collections`)

Vectors are growable arrays; maps/sets come in hash-based and ordered (BTree) variants.
Use `get` for lookups, `contains` for membership, and BTree types for sorted iteration.

```rust
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
let mut v = vec![3,1,2]; v.sort();
let mut h: HashMap<&str,i32> = HashMap::new(); h.insert("a",1);
let mut s: HashSet<i32> = HashSet::new(); s.insert(1);
let mut bt: BTreeMap<i32,&str> = BTreeMap::new(); bt.insert(1,"a");
let mut bs = BTreeSet::new(); bs.insert(3);
```

### Error handling (`ex_error_handling`)

Use `Result<T, E>` and the `?` operator to propagate errors. Handle outcomes with `match` or helpers.
Avoid panics in normal control flow; provide defaults with `unwrap_or` when appropriate.

```rust
use std::io::{Read, Result}; use std::fs::File;
fn read_to_string(p: &str) -> Result<String> { let mut f = File::open(p)?; let mut s = String::new(); f.read_to_string(&mut s)?; Ok(s) }
let r: Result<i32> = Ok(1);
```

### Functional programming (`ex_functional`)

Iterators enable expressive pipelines with `map`, `filter`, and `sum`. Chains are lazy until consumed.
Closures capture environment and compose well with iterator adapters.


```rust
let v = vec![1,2,3,4,5];
let s: i32 = v.iter().map(|x| x*2).filter(|x| x%3==0).sum();
let a = ["a","b","c"]; let j = a.iter().cloned().collect::<Vec<_>>().join("-");
let f = |x:i32| x+1; let y = f(9);
```