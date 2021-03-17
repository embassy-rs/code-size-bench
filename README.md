# Code size comparison

Sample projects for comparing code size of blocking vs async code.

All samples implement an UART echo in the simplest possible way: read 1 byte, write 1 byte.

- `hal-blocking`: uses the blocking driver from nrf52840-hal
- `async-embassy`: uses the async driver from embassy-nrf, running it with the embassy executor
- `async-block-on`: uses the async driver from embassy-nrf, running it with the simplest possible executor (basically equivalent to `nb::block_on`)

Results:

```
   text    data     bss     dec     hex filename
   1392       0      40    1432     598 out/async-block-on.elf
   2028       0     128    2156     86c out/async-embassy.elf
    540       0       4     544     220 out/hal-blocking.elf
```