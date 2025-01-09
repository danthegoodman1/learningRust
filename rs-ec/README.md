Go:
```
Shard by shard reconstruction:
Reconstruction time: 40.458µs

At once reconstruction:
Reconstruction time: 1.792µs

Large data reconstruction:
Encoding 64 MB of data with 4+2 encoding...
Encoding time: 3.079334ms
Reconstructing 2 missing shards...
Reconstruction time: 4.6365ms
Verification successful!
```

Rust:
```
Shard by shard reconstruction:
Reconstruction time: 3.834µs

At once reconstruction:
Reconstruction time: 2.208µs

Large data reconstruction:
Encoding 64 MB of data with 4+2 encoding...
Encoding time: 11.181208ms
Reconstructing 2 missing shards...
Reconstruction time: 9.762417ms
Verification successful!
```

Seems rust impl is better at small stuff, go is better at large stuff
