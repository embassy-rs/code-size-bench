# Code size comparison

Sample projects for comparing code size of blocking vs async code.

All samples implement an UART echo in the simplest possible way: read 1 byte, write 1 byte.

- `hal-blocking`: uses the blocking driver from nrf52840-hal
- `async-embassy`: uses the async driver from embassy-nrf, running it with the embassy executor
- `async-block-on`: uses the async driver from embassy-nrf, running it with the simplest possible executor (basically equivalent to `nb::block_on`)
- `drogue-device`: uses the executor and drivers from drogue-device

Results:

```
   text    data     bss     dec     hex filename
   1256       0      16    1272     4f8 out/async-block-on.elf
   1716       0     136    1852     73c out/async-embassy.elf
   9740       0    3056   12796    31fc out/drogue-device.elf
    540       0       4     544     220 out/hal-blocking.elf
```
