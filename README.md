# nrf54l15-flpr-pac
PAC for the nrf54l15-flpr microcontroller. 

## How to generate

First, get the SVD file from https://github.com/zephyrproject-rtos/hal_nordic/blob/master/nrfx/mdk/nrf54l15_flpr.svd or the root of this project

Install svd2rust and form, then run:

```bash
svd2rust -i nrf54l15_flpr.svd  --target riscv
rm -rf src/
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
```

