# How to Run

```shell
# Compile and run.
cargo run --release -- <rows> <columns> [-d delay = 25]

# Generate maze with 16 rows, 48 columns with the default delay of 25ms.
cargo run --release -- 16 48

# Generate maze with 16 rows, 48 columns with delay of 0ms (instant).
cargo run --release -- 16 48 -d 0
```

I tested that this works on at least Windows 10, Ubuntu and macOS.

![](example.gif)
