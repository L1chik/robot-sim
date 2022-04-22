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

***
# VRGlove
VR glove prototype: Arduino Nano, MPU6050, potentiometers. Finger bending tracking and hand rotation done.
_Todo: Simple VR headset, VR tracking system with cameras like in oculus._

```
cargo run --bin glove
```
**Note:** It won't start without correct serial port.