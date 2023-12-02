# Advent of Code 2023 - SAM D21 Edition

Advent of Code, but on the ATSAMD21G18, specifically an Arduino Nano
33 IoT, which I had in a drawer.

## Why would I do this to myself?

After I did [all of last year's AoC in under one
second](https://github.com/sulami/advent-of-code-2022), I wasn't
really sure what to do this year. I'll probably be busier than last
year as well, so I don't expect to actually finish in December (or at
all). Instead I figured it would be a fun to add some additional
constraints and see how far I can get, and maybe learn something along
the way.

I've been doing more and more embedded Rust over the last two years,
but I still feel out of my element compared to "regular" Rust.

So this is Rust, but without the `std` and `alloc` libraries, which
means no `String` or `Vec` or any kind of dynamically-sized,
heap-allocated data structures.

At first I was actually considering the Arduino Uno and its
ATmega328p, but 32 kB of program space is tricky if I want to include
the input in the binary via `include_str!()` (though I could drip-feed
it in via serial), and 2 kB of memory will be quickly prohibitively
small.

Instead I'm using the more powerful SAMD21. Processing speed probably
isn't too bad at 48 MHz, and 256 kB of program memory should be fine
as well. We will see how far 32 kB of memory get me.

For obvious reasons I will only run one day at a time.

## But does it work?

So far, yes. I use the Arduino serial console to grab the results,
which are printed on a loop.

## Build this yourself

Grab a bunch of dependencies:

- rustup
- arduino-cli
- just

```sh
arduino-cli core install arduino:samd
cargo install cargo-binutils
rustup component add llvm-tools-preview
rustup target add thumbv6m-none-eabi
```

Make sure to place your puzzle inputs in the `inputs` directory.

Build and flash with:

```sh
just build day-01
just upload /dev/tty.usbmodem123
```

The name of the serial port will vary based on the current phase of
the moon. Mine is either `101` or `1101`.
