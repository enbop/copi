# Copi

An easier way to control Raspberry Pi Pico.

*Extend IO for smart devices (Android/macOS/Windows/Linux) or enhance computing for Pico.*

## Build

Dependencies:
- Rust (Cargo)
- picotool (only for elf to uf2)

### Add Pico2 target support
```bash
rustup target add thumbv8m.main-none-eabihf
```

### Build Pico2 firmware
```bash
./build-firmware.sh
```

Output binary: `firmware-output/copi-firmware-pico2.uf2`

(Will use nightly to compile Pico2 firmware because the feature `#![feature(generic_arg_infer)]` is required)

### Build copi-cli
```bash
cargo build --bin copi
```

Output executable:
```
target/debug/copi
```

## Usage

```
$ target/debug/copi -h
```

```
Usage: copi [COMMAND]

Commands:
  list   List all connected boot pico devices
  flash  Flash copi firmware to the pico device
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Flash firmware to a Pico2

Hold the Pico2 BOOT button while connecting to the PC

#### List all connected boot pico devices
```bash
$ target/debug/copi list
```

Note the Pico2 mount location, e.g. `/Volumes/RP2350` (macOS) or `F:` (Windows)

#### Flash copi firmware to the pico device

```bash
$ target/debug/copi flash /Volumes/RP2350
```
or 
```bash
$ target/debug/copi flash F:
```

### Run Copi

```
target/debug/copi
```

```
Found device: "xxxxxx"
listening on 0.0.0.0:8899
```

### Blink the LED via a simple http

```
curl -X POST http://localhost:8899/gpio/output-init -H "Content-Type: application/json" -d '{"pin": 25, "value": true}'
```