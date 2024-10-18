# "An Inordinate Display" — talk and materials

This repo contains the notes (`inordinate.p.md`) and
materials (everything else) for my talk "An Inordinate
Display: "Blinky" In Embedded Rust.

(This talk was first delivered at the PDX Rust Meetup 1
August 2024.)

## Running This Code

* If needed, do the following to set up your tools:

      rustup target add thumbv7em-none-eabihf
      rustup component add llvm-tools
      cargo install cargo-binutils
      cargo install --force --locked probe-rs-tools

  (The `--force` is in case you have an old version of
  `probe-rs`: the tools and installation instructions have
  changed a few times.)

* Connect a BBC micro:bit v2 to a USB port on your
  machine. There are good instructions in
  <https://rust-embedded/discovery-mb2> for making this
  work.

* `cargo run --release --example foo` where `foo` is some code in the
  `examples` directory here. The first build may take a minute.
