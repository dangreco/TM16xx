# TM16xx

Platform-agnostic (via `embedded-hal`) Rust driver for `TM16xx` LED matrix modules.

## Feature Matrix

| Feature | Chip | Display | Buttons | Notes |
| ------- | ---- | ------- | ------- | ----- |
| `tm1628` | TM1628 | ✅ | ❌ | |

*✅ = full support, ❌ = no support, ⚠️ = partial + see notes*

## Usage

1. Install the crate along with the feature(s) for your chip(s):

```
$ cargo add tm16xx --features <chip>
```

2. Import the crate:

```rust
use tm16xx::prelude::*;
```

## Examples

- [esp32-tm1628](examples/esp32-tm1628) - ESP32-based counter with the TM1628