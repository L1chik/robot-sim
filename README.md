_Working on the refactoring..._

# Robot Sim
Industrial Robot Simulator, which currently provide Forward Kinematics. In the future I plan to realize:
* inverse kinematics
* real-time robot control
* global coordinate system
* movement types like SPTP, SLIN, SCIRC

```
cargo run --bin bluster
```

![Bluster robot-sim](/images/bluster.gif "Bluster")

Usage:
- `0 - 5`: joints switching (0 - static base, 1 - shoulder, 2 - lower arm, 3 - elbow, 4 - upper arm, 5 - wrist)
- `j`: negative rotation of the joint
- `u`: positive rotation of the joint
- `ctrl + horizontal mouse movement`: rotation of the joint

Demonstration of the latest version of the system with real robot control:

https://user-images.githubusercontent.com/55959772/233969782-b2e0af50-717d-4989-a837-63863593682f.mp4


***
# VRGlove
VR glove prototype: Arduino Nano, MPU6050, potentiometers, case is designed in Fusion360 and printed with Picasso designer x pro. Finger bending tracking and hand rotation done.
_Todo: Simple VR headset, VR inside-out tracking system with cameras like in oculus quest._

```
cargo run --bin glove
```
**Note:** It won't start without correct serial port. Change it in glove-example.rs init_port(name, baud_rate). _I will try to refactor everything to a separate library soon_


https://user-images.githubusercontent.com/55959772/164773258-8d154c9a-ef59-4cd2-a0da-82e23d7455d5.mp4




