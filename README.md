# gradcap-rs

My graduation cap running Rust to blink some lights

See the blog post here: [https://ericswpark.com/blog/2026/2026-05-12-my-graduation-cap-runs-rust/](https://ericswpark.com/blog/2026/2026-05-12-my-graduation-cap-runs-rust/)

## Install

Install dependencies of `avr-hal`:

```
sudo pacman -S avr-gcc avr-libc avrdude
```

Also install `micronucleus` for the ATTiny85 bootloader:

```
sudo pacman -S micronucleus
```

Then build this project and flash it:

```
cargo build --release
cd target/avr-none/release/

# Convert to hex format for micronucleus
avr-objcopy --output-target=ihex gradcap-rs.elf gradcap-rs.hex

# Flash with micronucleus
micronucleus --timeout 60 --run --no-ansi gradcap-rs.hex
```
