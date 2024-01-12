# libgpiod in Rust

[![github](https://img.shields.io/badge/github-katyo/gpiod--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/gpiod-rs)
[![MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![CI](https://img.shields.io/github/actions/workflow/status/katyo/gpiod-rs/ci.yml?branch=master&style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/gpiod-rs/actions?query=workflow%3ARust)

Rust crate for interfacing with Linux GPIO character devices.
This crate contains command-line tools based on gpiod library:

- `gpio` - based on std sync runtime
- `tgpio` - base on [tokio](https://tokio.rs/) async runtime
- `agpio` - based on [smol](https://github.com/smol-rs/smol) async runtime

It provides an interface to the Linux GPIO using the chardev module.
This interface involves calling [ioctl](https://man7.org/linux/man-pages/man2/ioctl.2.html) funcions which are unsafe and require some unintuitive variable mapping.
To ease this process, this crate provides a [Chip] struct which encapsulates the interface in safe Rust functions.
The functionality provided here is highly inspired by [libgpiod](https://git.kernel.org/pub/scm/libs/libgpiod/libgpiod.git/).

Since all functionality is dependent on Linux function calls, this crate only compiles for Linux systems.

## ABI compatibility

Both ABI v1 (linux >= 4.0) and v2 (linux >= v5.10) supported but edge detection implemented for v2 only.
Deprecated sysfs-based API (linux < 4.0) currently is not supported at all.

## Crates

- [gpiod-core](https://crates.io/crates/gpiod-core) - core abstractions and low level interface (not for end users)
- [gpiod](https://crates.io/crates/gpiod) - sync interface which supports synchronous operation only
- [tokio-gpiod](https://crates.io/crates/tokio-gpiod) - async interface for [tokio](https://tokio.rs/) fans
- [async-gpiod](https://crates.io/crates/async-gpiod) - async interface for various async runtimes

## Usage examples

Detect chips:
```sh
$ gpio detect
gpiochip0 [pinctrl-bcm2711] (58 lines)
gpiochip1 [raspberrypi-exp-gpio] (8 lines)
gpiochip2 [ftdi-cbus] (4 lines)
```

Get chip info:
```sh
$ gpio info gpiochip0
gpiochip0 [pinctrl-bcm2711] (58 lines)
         line    0:              "ID_SDA"        unused  input   active-high
         line    1:              "ID_SCL"        unused  input   active-high
         line    2:              "SDA1"  unused  input   active-high
         line    3:              "SCL1"  unused  input   active-high
         line    4:              "GPIO_GCLK"     unused  input   active-high
         line    5:              "GPIO5"         unused  input   active-high
         line    6:              "GPIO6"         unused  input   active-high
         line    7:              "SPI_CE1_N"     unused  input   active-high
         line    8:              "SPI_CE0_N"     unused  input   active-high
         line    9:              "SPI_MISO"      unused  input   active-high
         line    10:             "SPI_MOSI"      unused  input   active-high
         line    11:             "SPI_SCLK"      unused  input   active-high
         line    12:             "GPIO12"        unused  input   active-high
         line    13:             "GPIO13"        unused  input   active-high
         line    14:             "TXD1"  unused  input   active-high
         line    15:             "RXD1"  unused  input   active-high
         line    16:             "GPIO16"        unused  input   active-high
         line    17:             "GPIO17"        unused  input   active-high
         line    18:             "GPIO18"        unused  input   active-high
         line    19:             "GPIO19"        unused  input   active-high
         line    20:             "GPIO20"        unused  input   active-high
         line    21:             "GPIO21"        unused  input   active-high
         line    22:             "GPIO22"        unused  input   active-high
         line    23:             "GPIO23"        unused  output  active-high
         line    24:             "GPIO24"        unused  input   active-high
         line    25:             "GPIO25"        unused  input   active-high
         line    26:             "GPIO26"        unused  input   active-high
         line    27:             "GPIO27"        unused  input   active-high
         line    28:             "RGMII_MDIO"    unused  input   active-high
         line    29:             "RGMIO_MDC"     unused  input   active-high
         line    30:             "CTS0"  unused  input   active-high
         line    31:             "RTS0"  unused  input   active-high
         line    32:             "TXD0"  unused  input   active-high
         line    33:             "RXD0"  unused  input   active-high
         line    34:             "SD1_CLK"       unused  input   active-high
         line    35:             "SD1_CMD"       unused  input   active-high
         line    36:             "SD1_DATA0"     unused  input   active-high
         line    37:             "SD1_DATA1"     unused  input   active-high
         line    38:             "SD1_DATA2"     unused  input   active-high
         line    39:             "SD1_DATA3"     unused  input   active-high
         line    40:             "PWM0_MISO"     unused  input   active-high
         line    41:             "PWM1_MOSI"     unused  input   active-high
         line    42:             "STATUS_LED_G_CLK"      "led0"  output  active-high     [used]
         line    43:             "SPIFLASH_CE_N"         unused  input   active-high
         line    44:             "SDA0"  unused  input   active-high
         line    45:             "SCL0"  unused  input   active-high
         line    46:             "RGMII_RXCLK"   unused  input   active-high
         line    47:             "RGMII_RXCTL"   unused  input   active-high
         line    48:             "RGMII_RXD0"    unused  input   active-high
         line    49:             "RGMII_RXD1"    unused  input   active-high
         line    50:             "RGMII_RXD2"    unused  input   active-high
         line    51:             "RGMII_RXD3"    unused  input   active-high
         line    52:             "RGMII_TXCLK"   unused  input   active-high
         line    53:             "RGMII_TXCTL"   unused  input   active-high
         line    54:             "RGMII_TXD0"    unused  input   active-high
         line    55:             "RGMII_TXD1"    unused  input   active-high
         line    56:             "RGMII_TXD2"    unused  input   active-high
         line    57:             "RGMII_TXD3"    unused  input   active-high
```

Get line values:
```sh
$ gpio get gpiochip0 22 27
1 0
```

Set line values:
```sh
$ gpio set gpiochip0 21=1
1
```

Monitor line values:
```sh
$ gpio mon gpiochip0 22 27
line 27: rising-edge [408914.219966626]
line 27: falling-edge [408914.269983903]
line 27: rising-edge [408929.620077211]
line 27: falling-edge [408929.670091118]
```
