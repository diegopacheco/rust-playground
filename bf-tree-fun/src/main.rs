use bf_tree::BfTree;
use bf_tree::LeafReadResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::time::Instant;

fn bench_bftree_add(n: usize) -> u128 {
    let mut config = bf_tree::Config::default();
    config.cb_min_record_size(8);
    let tree = BfTree::with_config(config, None).unwrap();
    let start = Instant::now();
    for i in 0..n {
        let key = format!("key{:08}", i);
        let value = format!("value{:08}", i);
        tree.insert(key.as_bytes(), value.as_bytes());
    }
    start.elapsed().as_micros()
}

fn bench_bftree_retrieve(n: usize) -> u128 {
    let mut config = bf_tree::Config::default();
    config.cb_min_record_size(8);
    let tree = BfTree::with_config(config, None).unwrap();
    for i in 0..n {
        let key = format!("key{:08}", i);
        let value = format!("value{:08}", i);
        tree.insert(key.as_bytes(), value.as_bytes());
    }
    let mut buffer = [0u8; 1024];
    let start = Instant::now();
    for i in 0..n {
        let key = format!("key{:08}", i);
        let _ = tree.read(key.as_bytes(), &mut buffer);
    }
    start.elapsed().as_micros()
}

fn bench_bftree_remove(n: usize) -> u128 {
    let mut config = bf_tree::Config::default();
    config.cb_min_record_size(8);
    let tree = BfTree::with_config(config, None).unwrap();
    for i in 0..n {
        let key = format!("key{:08}", i);
        let value = format!("value{:08}", i);
        tree.insert(key.as_bytes(), value.as_bytes());
    }
    let start = Instant::now();
    for i in 0..n {
        let key = format!("key{:08}", i);
        tree.delete(key.as_bytes());
    }
    start.elapsed().as_micros()
}

fn bench_btreemap_add(n: usize) -> u128 {
    let mut tree: BTreeMap<String, String> = BTreeMap::new();
    let start = Instant::now();
    for i in 0..n {
        let key = format!("key{:08}", i);
        let value = format!("value{:08}", i);
        tree.insert(key, value);
    }
    start.elapsed().as_micros()
}

fn bench_btreemap_retrieve(n: usize) -> u128 {
    let mut tree: BTreeMap<String, String> = BTreeMap::new();
    for i in 0..n {
        let key = format!("key{:08}", i);
        let value = format!("value{:08}", i);
        tree.insert(key, value);
    }
    let start = Instant::now();
    for i in 0..n {
        let key = format!("key{:08}", i);
        let _ = tree.get(&key);
    }
    start.elapsed().as_micros()
}

fn bench_btreemap_remove(n: usize) -> u128 {
    let mut tree: BTreeMap<String, String> = BTreeMap::new();
    for i in 0..n {
        let key = format!("key{:08}", i);
        let value = format!("value{:08}", i);
        tree.insert(key, value);
    }
    let start = Instant::now();
    for i in 0..n {
        let key = format!("key{:08}", i);
        tree.remove(&key);
    }
    start.elapsed().as_micros()
}

fn bench_btreeset_add(n: usize) -> u128 {
    let mut tree: BTreeSet<String> = BTreeSet::new();
    let start = Instant::now();
    for i in 0..n {
        let key = format!("key{:08}", i);
        tree.insert(key);
    }
    start.elapsed().as_micros()
}

fn bench_btreeset_retrieve(n: usize) -> u128 {
    let mut tree: BTreeSet<String> = BTreeSet::new();
    for i in 0..n {
        let key = format!("key{:08}", i);
        tree.insert(key);
    }
    let start = Instant::now();
    for i in 0..n {
        let key = format!("key{:08}", i);
        let _ = tree.contains(&key);
    }
    start.elapsed().as_micros()
}

fn bench_btreeset_remove(n: usize) -> u128 {
    let mut tree: BTreeSet<String> = BTreeSet::new();
    for i in 0..n {
        let key = format!("key{:08}", i);
        tree.insert(key);
    }
    let start = Instant::now();
    for i in 0..n {
        let key = format!("key{:08}", i);
        tree.remove(&key);
    }
    start.elapsed().as_micros()
}

fn demo_bftree() {
    let mut config = bf_tree::Config::default();
    config.cb_min_record_size(4);
    let tree = BfTree::with_config(config, None).unwrap();
    tree.insert(b"key", b"value");
    let mut buffer = [0u8; 1024];
    let read_size = tree.read(b"key", &mut buffer);
    assert_eq!(read_size, LeafReadResult::Found(5));
    assert_eq!(&buffer[..5], b"value");
    println!("BfTree basic operations work correctly!");
}

fn run_benchmark(n: usize) {
    println!("--- {} elements ---", n);
    println!("{:<12} {:>12} {:>12} {:>12}", "Structure", "ADD (us)", "GET (us)", "REMOVE (us)");
    println!("{:-<12} {:->12} {:->12} {:->12}", "", "", "", "");
    
    let bftree_add = bench_bftree_add(n);
    let bftree_get = bench_bftree_retrieve(n);
    let bftree_remove = bench_bftree_remove(n);
    println!("{:<12} {:>12} {:>12} {:>12}", "BfTree", bftree_add, bftree_get, bftree_remove);
    
    let btreemap_add = bench_btreemap_add(n);
    let btreemap_get = bench_btreemap_retrieve(n);
    let btreemap_remove = bench_btreemap_remove(n);
    println!("{:<12} {:>12} {:>12} {:>12}", "BTreeMap", btreemap_add, btreemap_get, btreemap_remove);
    
    let btreeset_add = bench_btreeset_add(n);
    let btreeset_get = bench_btreeset_retrieve(n);
    let btreeset_remove = bench_btreeset_remove(n);
    println!("{:<12} {:>12} {:>12} {:>12}", "BTreeSet", btreeset_add, btreeset_get, btreeset_remove);
    println!();
}

fn main() {
    println!("=== BfTree Benchmark ===");
    println!();

    demo_bftree();
    println!();

    let sizes = [100, 1_000, 10_000, 100_000];
    for size in sizes {
        run_benchmark(size);
    }
}
