# Gain

The Extism plugin exports `gain`, which calculates a symmetric percentage change between two values. Swapping the values reverses the sign. Results are truncated toward zero to five decimal places.

## Build

Run from the workspace root with Rust and Cargo installed.

```sh
rustup target add wasm32-unknown-unknown
cargo build --locked --release --target wasm32-unknown-unknown -p gain
```

The build writes `target/wasm32-unknown-unknown/release/gain.wasm`.

## Call

Pass a JSON object with two numeric fields, `former` for the starting value and `later` for the ending value.

```sh
extism call target/wasm32-unknown-unknown/release/gain.wasm gain \
  --input '{"former":100,"later":125}'
```

The call returns `25.0` as a JSON number. Equal nonnegative values return `0.0`. Negative values, exactly one zero, and calculations that overflow return JSON `null`. Missing fields and invalid JSON cause the call to fail.

## Invert a gain

Call `gain_inv` with the starting value and a signed gain percentage to get the implied ending value.

```sh
extism call target/wasm32-unknown-unknown/release/gain.wasm gain_inv --input '{"former":125,"gain":-25}'
```

This returns `100.0`. If the percentage came from `gain`, truncation can prevent exact recovery of the original ending value. A zero starting value requires a zero gain. Invalid numeric combinations and results that overflow or underflow to zero return JSON `null`.

## Test

```sh
cargo test --locked -p gain
```
