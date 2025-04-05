set -e

if [ ! -d "firmware-output" ]; then
    mkdir firmware-output
fi

cd firmware/pico2
cargo build --release
cd ../..

picotool uf2 convert firmware/pico2/target/thumbv8m.main-none-eabihf/release/copi-firmware-pico2 -t elf firmware-output/copi-firmware-pico2.uf2