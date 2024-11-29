# Fleet DM

Fleet (device manager)

REF; https://github.com/fleetdm/fleet/tree/main

The developers of Fleet have tooling and server/client software for device management of various operating systems.
I became confident that creating my own MDM server for windows is "easily" possible after seeing their PoC (which I couldn't get working on Windows containers).

A big part of the code in this repository is inspired by fleet's code. My intention to have the end result run as a windows service made me choose for an (re)implementation
in Rust, instead of blatantly copying required Go code. Also fleet's code is abstracted towards bigger goals than 'straight forward self-managing computer' which would require
me to cut out a lot of functionality if I copied.