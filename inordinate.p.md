# An Inordinate Display

"Blinky" In Embedded Rust

Bart Massey <bart.massey@gmail.com>

## Embedded

* Concentrates on devices that are typically "too small" to
  comfortably run a full modern OS on

* Commonly aimed at System-on-Chip (SoC) boards. Package may contain:
      * Micro-Controller Unit (MCU)
      * flash memory
      * RAM
      * many "peripherals": dedicated hardware for common tasks

* There are thousands of kinds of these out there, and they
  are in *everything*: probably a hundred in this room right
  now

## Bare Metal

* The line as to what constitutes "Embedded" software is
  quite blurry
  
  * Often supports "hard real time"
  * Usually rather constrained by speed and memory
  * Almost always supports "direct" access to on-board hardware
  * May have an embedded "Real Time Operating System" (RTOS) kernel

* In "Bare Metal" embedded, there is no OS kernel (although
  there may be some sort of organizing "runtime"). The
  programmer controls *every* instruction that runs on the
  device.

## Bare Metal Rust

* A Rust embedded framework used by almost everybody was
  pioneered by Jorge Aparicio a decade ago. There's a Rust
  Project Working Group

* Key advantages over C/C++:

  * Rust's easy cross-compilation to abstract away MCU
  * Rust traits to define cross-platform interfaces
  * Cargo for easy mix-and-match
  * Almost fully open-source non-proprietary ecosystem
  * Well-defined zero-cost abstractions
  * Rust's famous safety guarantees (even in `unsafe` code)
  * Healthy and supportive community

* Key disadvantages: 

  * Less pre-built stuff out there
  * Rust is "hard"

## Blinky

* The "hello world" of bare-metal embedded

* Blink an LED on your board on and off once per second

* (Yes, every board has a blinkable LED)

* Shows you can:

  * Get your boot code running on the board

  * Wiggle a pin on the MCU in a controlled way

  * Time things properly

* **Inordinately** complicated, but not *that* bad

## Our Blinky

* <https://google.github.io/comprehensive-rust/bare-metal>
  gives a really good quick account of bare-metal Rust. But
  no blinky
  
* We will follow
  <https://github.com/rust-embedded/discovery-mb2>
  which I just finished a big revision of

  Specifically, the "Hello World" chapter does blinky

* <https://github.com/BartMassey/inordinate> contains this
  talk and the source code for today

## Meet Our Hardware

* The BBC micro:bit v2 is a board for kids to play with. It
  has an Nordic nRF52833 ARM processor and a bunch of
  supporting stuff
  
* 512KB flash, 128KB memory. Fairly big, but you ain't
  likely running Linux. 64MHz CPU. Lots of peripherals on
  and off the MCU

* "Standard" environments include MicroPython (written in C)
  and a "bricks programming" environment (mostly Javascript)

* We use `probe-rs` and friends to load programs over
  USB. See the `README`

## `no_std`, `no_main`

* When you write a "normal" Rust application

  * It is linked against the Rust `std` library. We don't
    (normally) have one, for good reasons

  * Some code is inserted to set things up and then call
    `main()`. (Yes, Rust has "life before main" in practice.)

* When you write a bare-metal Rust application

  * You write "linker scripts" to specify what your
    executable will look like

  * You supply things Rust `core` (but not `std`) needs to
    be able to work

* See <https://github.com/pdx-cs-rust-embedded/nostd-rs> for
  an os-based version of this

* See `examples/no-std.rs` for our framework


## Infrastructure

Things to take a look at now:

* `rust-toolchain.toml`
* `Cargo.toml`
* `.cargo/config.toml`
* `Embed.toml`

## Monitoring and Debugging

* Source-level debugging is now pretty much expected in bare
  metal. MB2 Discovery book describes how to use `cargo
  embed` with `gdb`

* Usually just want to see what's happening. This is
  normally a hardware serial port or some equivalent USB
  protocol. We will use Seger's RTT for our hardware
  (`examples/rtt.rs`)

## LED hardware

* We want to turn on the LED at the upper-left corner of our
  "display": row 1, column 1

* The "positive wire" ("anode") of the LED is attached to a
  General-Purpose I/O (GPIO) row1 "pin" on the
  nRF52833. The "negative wire" ("cathode") is attached to a
  GPIO column1 pin

* After spending a half-hour (seriously) you will figure out
  that the nRF52833 names for the MB2 row1 and column1 GPIO
  pins are P0.21 and P0.28 (seriously)

* You find out or change what the hardware is doing by
  reading and writing hardware-specific locations "in memory"
  known as "device registers"

* We use the Rust `nrf52833-hal` Hardware Abstraction Layer
  crate to alter these registers. This gives us indirect
  access to the `nrf52833-pac` Peripheral Access Crate

## Blinky Step 1: Light The LED

* Ok, it's code time (`examples/light.rs`)

* There are "ownership semantics" and "typestate patterns" here

* It doesn't blink yet: just getting it turned on is a thing

## Blinky Step 2: Turn The LED On and Off

* After some digging in the documentation, find some methods
  to change pins' state (awkward)

* So just loop flashing on and off! (`examples/blinky.rs`)

* Note the `embedded_hal` reference: these are traits any
  Rust embedded HAL might (should?) implement

* We put the flashing LED next to a solid one, because reasons

* Ah, that won't work! Now it just looks slightly darker

* (Try turning it around and compiling in release mode!)

## Blinky Step 3: Wait 0.5s

* We need to wait 500ms (0.5s) between LED changes

* "Obvious" plan: sit and spin — feels transgressive
  (`examples/spin-wait.rs`)

* Where did that iteration count come from? Roughly 8 cycles
  per iteration (measured, theoretical), 64MHz so about 16ns
  per cycle

* What's the `nop()` for? Inserts a must-execute machine
  instruction into the loop

* Is this any good? Heck no

  * debug vs release
  * precision
  * burning power and cycles

## Blinky Step 4: Wait *Exactly* 0.5s

* MCUs have hardware "timers". We will use HAL (`examples/blinky.rs`)

* The `DelayNs` trait is portable and applies to timers,
  `SYST`, other architectures, etc

* This gets you 500ms pretty exactly, blocks internally
  (`wfi`). Can do concurrency with timer interrupts too

## We Win!

* After all that, we're ready to do start some serious
  business with our MB2. It's way easier the second time

## Bonus: BSP

* `microbit-v2` Board Support Package (crate) provides
  cleaner fancier interfaces

* Let's use the "blocking display" interface (`examples/bsp.el`)

## Resources

* <https://docs.rust-embedded.org/> Base place for Rust
  Embedded WG resources

* <https://github.com/rust-embedded/discovery-mb2> The
  "new" not-yet-debugged not-yet-fully-published MB2 book

* <https://tech.microbit.org/> Lots of tech information
  for MB2

* <https://github.com/pdx-cs-rust-embedded> Many of my examples
  in various states of repair and usefulness

## Acknowledgements

* Thanks to the Rust Embedded WG and the folks of the Rust
  and Rust Embedded community

* Thanks to the patience and help of my students, friends
  and colleagues
