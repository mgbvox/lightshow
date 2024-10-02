# Microbit Lightshow!

Wherein I learn embedded rust (on the micro:bit v2) in a simple 
led-array proof-of-concept project.

### Build + Embed it:
Connect a micro:bit v2 device to your machine via usb, and:
```bash

cargo embed --target thumbv7em-none-eabihf
```

The led array should be twinkling now:

![twinkle](/assets/twinkle.gif)