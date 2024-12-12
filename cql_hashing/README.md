# CQL Hashing

Trying to match the hashing algo of CQL

```

CREATE TABLE composite_test (
    a text,
    b blob,
    c timestamp,
    value text,
    PRIMARY KEY ((a, b, c))
);

INSERT INTO composite_test (a, b, c, value)
VALUES ('hello', 0x0102, '2022-01-01', 'test');


cqlsh:test_ks> SELECT token(a, b, c) FROM composite_test;

 system.token(a, b, c)
-----------------------
   4701541058813499942


$ cargo run

token: 4701541058813499942
```

With scylla
