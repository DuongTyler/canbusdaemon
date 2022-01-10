# Arduino-Openauto Canbus Daemon
This code aims to connect an arduino uno using an inland canbus hat with a raspberry pi running openauto pro.

The canbus IDs are for a 2012 Honda Civic Sedan. I don't intend on trying to decipher any other car canbus unless someone wants to gift me a car :D.

## !!! USE THIS SOFTWARE AT YOUR OWN RISK. I TAKE NO RESPONSIBILITY FOR YOUR DUMB ASS BRICKING YOUR CAR. YOU HAVE BEEN WARNED. !!!

# How to use this software
Inspect the arduino code, make sure it is using the right baudrate and spi pins for your arduino and modify if needed. The arduino transmits a canbus sensor struct, but it's transmitted in a little endian byte order, so if you're modifying something with the communication between the arduino and the pi, keep this in mind.

Compile the rust program for the raspberry pi. It depends on libxdo for keyboard events. You can change this if you'd like. It's in ``src/event_listener.rs``.
There is plenty of room to add custom event handling. Just add an id:function entry to the `honda_sensor_hashmap` in `src/main.rs` and create the new function. Easy peasy.

Canbus ids are in `main.rs` and can be changed as you like. Maybe one day I'll make the program read from a config file that maps canbus ids to scripts in a directory but eh, if someone wants to do that feel free to open a PR.

# Additional Notes (for hacking your car)
I'm pretty sure that there's no gear sensor (for detecting reverse gear) that communicates on the BCAN-BUS on Honda vehicles. More than likely it's on the FCAN-BUS where all the other high-speed important information lies. I don't want to tap into that but if you really wanted to you can try tapping into the F-CAN bus for that info. I just used a 12v relay to short the 3.3v pin and the gpio pin for the backup signal on openauto.

Also, I'm really stupid, so this might not be as efficient as it could be. If you feel like giving me pointers on how to code more efficiently, I'd love to hear feedback. I'm still a student with much to learn.

Oh, and this code is under the MIT license