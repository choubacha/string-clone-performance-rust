# String Clone performance test

Runs a quick test of how fast string cloning happens in rust for various
sized strings.

```
$ ./target/release/string-clone-performance
              100 rs:     25686529.86 i/sec
             1000 rs:     22013534.46 i/sec
           10,000 rs:      6876453.71 i/sec
          100,000 rs:       254827.46 i/sec
```
