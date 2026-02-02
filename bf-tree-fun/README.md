### Build

```bash
cargo build --release
```

### Run

```bash
./run.sh
```

### Result

```
=== BfTree Benchmark ===

BfTree basic operations work correctly!

--- 100 elements ---
Structure        ADD (us)     GET (us)  REMOVE (us)
------------ ------------ ------------ ------------
BfTree                 89           48           58
BTreeMap               50           24           28
BTreeSet               32           24           24

--- 1000 elements ---
Structure        ADD (us)     GET (us)  REMOVE (us)
------------ ------------ ------------ ------------
BfTree               1188          243          306
BTreeMap              378          224          209
BTreeSet              280          219          179

--- 10000 elements ---
Structure        ADD (us)     GET (us)  REMOVE (us)
------------ ------------ ------------ ------------
BfTree               8952         2185         2150
BTreeMap             3193         1637         1458
BTreeSet             2194         1650         1251

--- 100000 elements ---
Structure        ADD (us)     GET (us)  REMOVE (us)
------------ ------------ ------------ ------------
BfTree              77070        25150        23968
BTreeMap            33071        18022        15693
BTreeSet            24093        17534        12880
```