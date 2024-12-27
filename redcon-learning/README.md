Timing results from release build:

```
Serving at 127.0.0.1:6380
["SET", "my_key", "42", "NX", "GET", "EX", "60"]
parsed: Ok(Set { key: "my_key", value: "42", options: SetOptions { nx: true, xx: false, get: true, expiration: Some(Ex(60)) } })
Parsing time: 3.958µs
["GET", "my_key"]
["SET", "my_key", "42"]
parsed: Ok(Set { key: "my_key", value: "42", options: SetOptions { nx: false, xx: false, get: false, expiration: None } })
Parsing time: 1.25µs
["GET", "my_key"]
["SET", "my_key", "42"]
parsed: Ok(Set { key: "my_key", value: "42", options: SetOptions { nx: false, xx: false, get: false, expiration: None } })
Parsing time: 792ns
["GET", "my_key"]
["SET", "my_key", "42", "NX", "GET", "EX", "60"]
parsed: Ok(Set { key: "my_key", value: "42", options: SetOptions { nx: true, xx: false, get: true, expiration: Some(Ex(60)) } })
Parsing time: 5.166µs
["GET", "my_key"]
["SET", "my_key", "42", "NX", "GET", "EX", "60"]
parsed: Ok(Set { key: "my_key", value: "42", options: SetOptions { nx: true, xx: false, get: true, expiration: Some(Ex(60)) } })
Parsing time: 2.208µs
["GET", "my_key"]
["SET", "my_key", "42", "NX", "GET", "EX", "60"]
parsed: Ok(Set { key: "my_key", value: "42", options: SetOptions { nx: true, xx: false, get: true, expiration: Some(Ex(60)) } })
Parsing time: 2.167µs
["GET", "my_key"]
["SET", "my_key", "42", "NX", "GET", "EX", "60"]
parsed: Ok(Set { key: "my_key", value: "42", options: SetOptions { nx: true, xx: false, get: true, expiration: Some(Ex(60)) } })
Parsing time: 2.25µs
["GET", "my_key"]
["SET", "my_key", "42", "NX", "GET", "EX", "60"]
parsed: Ok(Set { key: "my_key", value: "42", options: SetOptions { nx: true, xx: false, get: true, expiration: Some(Ex(60)) } })
Parsing time: 2.166µs
["GET", "my_key"]
```
