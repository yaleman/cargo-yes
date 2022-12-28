# cargo-yes

Yelling yes into the void...

```shell
 cargo run --release |  pv --buffer-size 102400000 --stop-at-size --size 1000000000000 > /dev/null
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/cargo-yes`
61.1GiB 0:00:09 [7.34GiB/s] [=====>                                       ]  7% ETA 0:02:05
```

That'll do I guess :)
