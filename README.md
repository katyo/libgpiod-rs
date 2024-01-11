# libgpiod in Rust

[![github](https://img.shields.io/badge/github-katyo/gpiod--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/gpiod-rs)
[![crate](https://img.shields.io/crates/v/gpiod.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/gpiod)
[![docs](https://img.shields.io/badge/docs.rs-gpiod-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/gpiod)
[![MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![CI](https://img.shields.io/github/actions/workflow/status/katyo/gpiod-rs/ci.yml?branch=master&style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/gpiod-rs/actions?query=workflow%3ARust)

Rust crate for interfacing with Linux GPIO character devices.

It provides an interface to the Linux GPIO using the chardev module.
This interface involves calling [ioctl](https://man7.org/linux/man-pages/man2/ioctl.2.html) funcions which are unsafe and require some unintuitive variable mapping.
To ease this process, this crate provides a [Chip] struct which encapsulates the interface in safe Rust functions.
The functionality provided here is highly inspired by [libgpiod](https://git.kernel.org/pub/scm/libs/libgpiod/libgpiod.git/).

Since all functionality is dependent on Linux function calls, this crate only compiles for Linux systems.

## ABI compatibility

Both ABI v1 (linux >= 4.0) and v2 (linux >= v5.10) supported but edge detection implemented for v2 only.
Deprecated sysfs-based API (linux < 4.0) currently is not supported at all.

## Crates

- [gpiod-core](https://crates.io/crates/gpiod-core) - core abstractions and low level interface (not for end users)
- **[gpiod](https://crates.io/crates/gpiod)** - sync interface which supports synchronous operation only
- [tokio-gpiod](https://crates.io/crates/tokio-gpiod) - async interface for [tokio](https://tokio.rs/) fans
- [async-gpiod](https://crates.io/crates/async-gpiod) - async interface to use with non-tokio async runtimes

## Usage examples

Input values:

```rust,no_run
use gpiod::{Chip, Options, Masked, AsValuesMut};

fn main() -> std::io::Result<()> {
    let chip = Chip::new(0)?; // open chip

    let opts = Options::input([27, 3, 11]) // configure lines offsets
        .consumer("my-inputs"); // optionally set consumer string

    let inputs = chip.request_lines(opts)?;

    // get all three values
    let values = inputs.get_values([false; 3])?;

    println!("values: {:?}", values);

    // get second value only
    let values = inputs.get_values([None, Some(false), None])?;

    println!("values: {:?}", values);

    // get values via bits
    let values = inputs.get_values(0u8)?;

    println!("values: {:#b}", values);

    // get only second value via bits
    let values = inputs.get_values(Masked::<u8>::default().with(1, Some(false)))?;

    println!("values: {:#b}", values);

    Ok(())
}
```

Output values:

```rust,no_run
use gpiod::{Chip, Options, Masked, AsValuesMut};

fn main() -> std::io::Result<()> {
    let chip = Chip::new("gpiochip0")?; // open chip

    let opts = Options::output([9, 21]) // configure lines offsets
        .values([false, true]) // optionally set initial values
        .consumer("my-outputs"); // optionally set consumer string

    let outputs = chip.request_lines(opts)?;

    // set all two values
    outputs.set_values([true, false])?;

    // set second value only
    outputs.set_values([None, Some(false)])?;

    // set values from bits
    outputs.set_values(0b01u8)?;

    // set only second value from bits
    outputs.set_values(Masked::<u8>::default().with(1, Some(true)))?;

    Ok(())
}
```

Monitor values:

```rust,no_run
use gpiod::{Chip, Options, EdgeDetect};

fn main() -> std::io::Result<()> {
    let chip = Chip::new("gpiochip0")?; // open chip

    let opts = Options::input([4, 7]) // configure lines offsets
        .edge(EdgeDetect::Both) // configure edges to detect
        .consumer("my-inputs"); // optionally set consumer string

    let mut inputs = chip.request_lines(opts)?;

    loop {
        let event = inputs.read_event()?;

        println!("event: {:?}", event);
    }

    Ok(())
}
```
