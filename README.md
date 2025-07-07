## `here!()` macro used for log backtracing

```rust
use anyhow::{Context, Result, anyhow};
use mini_macro::here;

fn produce_err_inside2() -> Result<()> {
    Err(anyhow!("ErrHere1")).context(here!())
}

fn produce_err_inside1() -> Result<()> {
    produce_err_inside2().context(here!("FailedInside1"))
}

fn produce_err_outside() -> Result<()> {
    produce_err_inside1().context(here!("OutsideFailed"))
}

fn main() {
    if let Err(e) = produce_err_outside() {
        println!("Err: {:?}", e);
    }
}
```

```
Err: src/main.rs:13:35: OutsideFailed

Caused by:
    0: src/main.rs:9:35: FailedInside1, msg
    1: src/main.rs:5:38
    2: ErrHere1
```

## `async_loop_until_success` macro used for waiting for async operations

> dependency: tokio, tracing

```rust
let res = async_loop_until_success!(self.clone().doing_something());
```
