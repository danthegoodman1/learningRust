# Rust big tree testing

Testing the performance of single large `BTreeMap` vs partitioned by key ranges.

## Testing strategy

Load up the tree with 50GB of keys of random UUIDs, then getting 100,000 random of them. This has an `Arc<Mutex>` to simulate the penalty of concurrent access

Then create 100 individual trees within a `HashMap`

Not only does this have the extra `HashMap`, but it also has an extra layer of `Arc<Mutex>` because we have to have that on the `HashMap`, but also on each child `BTreeMap`.

## Test results

Run on an Macbook Pro M3 Max, 128GB memory, Rust 1.83, macOS 14.5

### 10M UUIDs, 100k lookups

```
Generating 10000000 random UUIDs...

Testing single large BTreeMap...
Single tree query time: 54.730542ms

Generating 10000000 random UUIDs...

Testing partitioned BTreeMaps...
Partitioned maps query time: 48.533833ms
```

A single BTreeMap is about 12.5% slower for 100M UUIDs and 100k lookups

### 100M UUIDs, 1M lookups

```
Generating 100000000 random UUIDs...

Testing single large BTreeMap...
Single tree query time: 685.393708ms

Generating 100000000 random UUIDs...

Testing partitioned BTreeMaps...
Partitioned maps query time: 672.563916ms
```

I wonder if this finally started to spill to disk, since the query times went up sub-linearly.
