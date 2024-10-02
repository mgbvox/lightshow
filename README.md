# Microbit Lightshow!

Wherein I learn embedded rust (on the micro:bit v2) in a simple 
led-array proof-of-concept project. Adapted from https://docs.rust-embedded.org/,
with updated logic and config for compatibility with the latest microbit-v2 API.

### Build + Embed it:
Connect a micro:bit v2 device to your machine via usb, and:
```bash

cargo embed --target thumbv7em-none-eabihf
```

The led array should be twinkling now:

![twinkle](/assets/twinkle.gif)