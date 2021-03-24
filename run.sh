#!/bin/bash
set -euo pipefail

rm -rf out
mkdir out

for i in async-block-on async-embassy hal-blocking drogue-device; do
    (cd $i; cargo build --release)
    cp $i/target/thumbv7em-none-eabi/release/echo out/$i.elf
    llvm-objdump --disassemble out/$i.elf > out/$i.s
done

llvm-size out/*.elf