# Code size comparison

Sample projects for comparing code size of blocking vs async code.

Results:

```
   text    data     bss     dec     hex filename
   1392       0      40    1432     598 out/async-block-on.elf
   2028       0     128    2156     86c out/async-embassy.elf
    540       0       4     544     220 out/hal-blocking.elf
```