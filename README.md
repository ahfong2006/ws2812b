# `ws2812b`

> WS2812B LED ring controlled via a serial interface

![Setup](/assets/setup.jpg)

![Crescendo sequence](/assets/crescendo.gif)

<p style="text-align:center">
  <img alt="Crescendo sequence" src="/assets/crescendo.gif"/>
</p>

## Part list

- [24x WS2812B LED ring](https://www.aliexpress.com/wholesale?SearchText=ws2812b+24)
- 5V power supply
- [Blue pill](http://wiki.stm32duino.com/index.php?title=Blue_Pill)
- [HC-06](https://www.aliexpress.com/wholesale?SearchText=hc-06) Bluetooth
  module or any other serial interface

## Components

What you'll find in this repo.

- [firmware](/firmware). The code that runs on the LED driver.
- [logcat](/logcat). Tool to parse logs generated by the LED driver.
- [sequence](/sequence). Tool to send commands to the LED driver.
- [shared](/shared). Code shared between all the components.

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
