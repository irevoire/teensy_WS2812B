# Teensy WS2812B

This repository contains a code sample to use the WS2812B led with our [teensy crate](https://github.com/irevoire/teensy).

# Setup
You should connect the teensy to a 5V usb source.
- Connect the WS2812B **VDD** to the **VIN** port on the teensy.
- Connect the WS2812B **VSS** to any ground **GND** port on the teensy.
- Connect the WS2812B **DIN** to the **led** (the **pin 13**) of the teensy.

Setup the rust compiler for this repository:
```
sh ./configure.sh
```

# Run the code
```
make flash
```
