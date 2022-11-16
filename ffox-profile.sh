perf record -g -F 999 -- target/release/rust-jvm -r $1
perf script -F +pid > /tmp/test.perf