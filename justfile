# mode: makefile

build day:
	cargo build --release --target thumbv6m-none-eabi -F {{day}}
	rust-objcopy -O binary target/thumbv6m-none-eabi/release/advent-of-code-2023 target/aoc23.bin

upload serial_port:
    arduino-cli upload -i target/aoc23.bin -b arduino:samd:nano_33_iot -p {{serial_port}}