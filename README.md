# usbsdmux

[![API](https://docs.rs/usbsdmux/badge.svg)](https://docs.rs/usbsdmux)
[![Crate](https://img.shields.io/crates/v/usbsdmux.svg)](https://crates.io/crates/usbsdmux)

Program and library to control the [USB-SD-Mux device](https://shop.linux-automation.com/usb_sd_mux-D02-R01-V02-C00)

The program has the same interface as the original [python control program](https://github.com/linux-automation/usbsdmux) by the manufacturer:
```
$ usbsdmux /dev/sg0 host
$ usbsdmux /dev/sg0 dut
$ usbsdmux /dev/sg0 off
```

If you want to run the program as normal user you can set the setuid flag:

```
$ sudo chown root /path/to/usbsdmux
$ sudo chmod u+s /path/to/usbsdmux
```
