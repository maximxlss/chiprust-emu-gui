# Experimental GUI for [chiprust-emu](https://github.com/Maxxls/chiprust-emu)
[Pixels](https://crates.io/crates/pixels) based GUI for my emulator.
## Usage:
    chiprust-emu-gui.exe [options] [source]

Flags:
- -h, --help       Prints help information
- -V, --version    Prints version information

Options:
- -c, --cpu <frequency>      Sets a custom cpu frequency. If zero, instructions would be executed ASAP. [default: 840]
- -d, --draw <frequency>     Sets a custom draw frequency. Recommended to keep equal to the monitor refresh rate. [default: 60]
- -s, --speed <frequency>    Sets a custom speed (actually timers' tick frequency). If zero, timers would be decremented ASAP. [default: 60]
- --tone <frequency>     Sets a custom audio tone [default: 900]
- -t, --transparent     Makes background transparent.

ARGS:
- [source]    Sets the rom file to execute. Default: included tetris rom

## Problems:
- Uses wgpu, so doesn't support wasm. (Probably will change)
- Badly organized. (Will be fixed?)
