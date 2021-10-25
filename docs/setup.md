# Setup
This documentation is intended to help you through the steps of setting up
sys-hidplus-client-rs on your computer for general use. General use includes
using normally-unsupported controllers or playing on your Switch remotely.

# sys-hidplus
Although I refer to my client as an alternative client, the latest release
requires my fork of sys-hidplus in order to have 8-controller support. You'll 
have to install it on your Switch first.

## Requirements
- A Nintendo Switch with Atmosphere CFW installed.
  - The latest Switch firmware and atmosphere versions are recommended. You may
    encounter problems otherwise.
- A file archiver such as 7-Zip, WinRAR, or some equivalent.
- A way of moving files to your Switch's SD card.

## Directions
1. Download the latest version of the sys-hidplus sysmodule from
   [the releases page](https://github.com/kenesu-h/sys-hidplus/releases).
2. Using a file archiver, extract the contents of the latest release's archive
   into the root of your Switch's SD card. Overwrite files if asked.
3. If your Switch is already running, reboot into CFW.

# sys-hidplus-client-rs

## Requirements
- A file archiver such as 7-Zip, WinRAR, or some equivalent.

## Directions
1. Download the latest version of the client from
   [the releases page](https://github.com/kenesu-h/sys-hidplus-client-rs/releases).
   Specifically, download the archive for your specific operating system.
   - Mac OS and Linux releases may not be available since I may not have access
     to those operating systems. If you use either of these, please consult
     [Build](./build.md) for directions on building the client for your 
     operating system. Feel free to contact me directly if you want help.
2. Using a file archiver, extract the contents of the latest release's archive
   into a folder. Overwrite files if asked.

# Usage
## Startup
Follow the steps in this section every time you want to startup the client.

### Windows
You can run the executable directly without any issues.

### All Operating Systems
If you cannot directly run the executable or you want to run the command line
interface, you're going to have to run the client using your OS's terminal/shell.

#### Directions
1. Open your terminal/shell and navigate to the directory containing your
   executable.
2. Run the following: `./client-rs`
  - For Windows, you may have to do `./client-rs.exe` or just `client-rs.exe`.
  - If the above doesn't work, you may have to run the following to make it
    executable: `chmod +x client-rs`. Try to run the executable again
    afterwards.

If you want to run the command line interface, run the executable with the
following argument: `-- cli`.

## First-Time Setup
On your first startup, you will have to set your Switch's IP before you can make
a connection to it.

### GUI
If you're using the client GUI, simply fill out the text field for the Switch's
IP and click "Save".

### Command Line
If you're using the command line, set it by using the command
`set_server_ip 'server_ip'`, where 'server_ip' is replaced with the IP of your
Switch.

### config.toml
If for whatever reason you don't want to deal with the GUI or the command line,
you can edit `config.toml`, which should be generated in the same directory as
your executable.


## Making a Client-Switch Connection
Follow the steps in this section every time you want to connect your client to
your Switch. This will allow you to begin connecting your controllers.

**These steps assume that your computer and Switch are on the same network. If
they aren't, you may not be able to make a connection. An ethernet connection
for both will likely lead to the best results.**

### GUI
If you're using the client GUI, simply click the "Start Client" button.

### Command Line
If you're using the command line, use the command `start`.


## Connecting Controllers
Plug into your computer the controllers you want to use on your Switch. Most
controllers should be plug-and-play, but check out the
[Controller Compatibility section](#controller-compatibility) to see if your
controller needs additional steps to setup.

When you want to activate and map a controller, press your right bumper.

Activating a controller will assign it to the first available slot, and will not
override slots that are already connected. You may have to make a few button
inputs for your controller to be recognized by your Switch.

Repeat this process for every controller you want to connect.


## Disconnecting Controllers
Disconnecting controllers is as easy as unplugging them from your computer. The
slot they occupied will become usable by other controllers. You can reconnect
and reactivate your controller at any time.


## Exiting
**I strongly recommend against using the usual X button in the top-right of the
window to close the client. Your controllers will be left connected to your
Switch and take up wireless controller slots if you don't.**

### GUI
If you're using the client GUI, just click the "Exit" button.

### Command Line
If you're using the command line, use the `exit` command.

Either way, if you had controllers connected to your Switch, the client will
appear to freeze for about 3 seconds. After those 3 seconds, it should close
itself.

If you ran the client via terminal/shell, feel free to close it at this point.


# Extras

## Additional Configuration
The client offers additional configuration to your controllers, such as changing
a slot to a sideways Joy-Con and giving it artificial input delay.

### Controller Types

#### GUI
If you're using the client GUI, you're currently unable to change a controller's
type since the GUI library I used doesn't support dropdown menus.

#### Command Line
If you're using the command line, use the `set_switch_pad 'i' 'switch_pad'`
command, where 'i' represents the "index" of the slot you want to change, and
'switch_pad' is any one of the following:
```
Disconnected
ProController
JoyConLSide
JoyConRSide
```
The "index" is just the slot # - 1. For example, if you wanted to change a
controller in slot 2 to a sideways left Joy-Con, run:
`set_switch_pad 1 JoyConLSide`

Either way, a few warnings:
- If you run this while the client is running, you may have to restart it (you
  can literally just stop then start) for the changes to take effect.
- The SL and SR buttons on sideways Joy-Cons will not work. This is
  unfortunately out of our control unless libnx implements it, but
  [it doesn't look like that'll happen](https://github.com/switchbrew/libnx/issues/567).
  As a result, it's impossible at the moment to implement full sideways Joy-Con
  support.
- A sideways Joy-Con may be connected as an incomplete pair of Joy-Cons. While I
  haven't tested this with games that force you to use sideways Joy-Cons as
  opposed to a pair, there may still be problems with this.

### Input Delay
Similarly, you can add artificial input delay to a controller slot.

#### GUI
Like usual, use the respective text field to set a controller's input delay.

#### Command Line
If you're using the command line, use the `set_input_delay 'i' 'input_delay'`
command. 'i' must be the index of the slot you want to change, and 'input_delay'
must be a positive number.

Unlike changing controller types, you don't need to restart the client for the
changes to work.

### Deadzones
You can also set the analog stick deadzones of a controller slot.

#### GUI
Like usual, use the respective slider to set a controller's deadzone.

#### Command Line
If you're using the command line, use the `set_left_deadzone 'i' 'deadzone'` or
`set_right_deadzone 'i' 'deadzone'` commands as needed. 'i' must be the index of
the slot you want to change, and 'deadzone' should be a decimal between 0.0 and
1.0.

You don't need to restart the client for this to work either.

## Controller Compatibility
These controllers are confirmed to be practically plug-and-play:
- Xbox 360 Controllers
- Xbox One Controllers
- PS4 Controllers
  - The touchpad isn't bound to anything, but everything otherwise works.
- Switch Pro Controllers
  - You may need Steam open for Pro Controllers to work, along with
    configuration for them enabled in Big Picture.
  - The capture button isn't bound to anything. This is out of my control and
    can't be changed.
- Joy-Cons

Kemosahbee has confirmed the following:
- Razer Kishis
  - They work as long as the phone is connected via Parsec. Check out
    [the Parsec page](./parsec.md) for information on setup.

ConspiracyFactualist has confirmed the following:
- Controllers connected via a MAYFLASH Magic-NS adapter.

These controllers will require external software:
- Keyboard (and/or Mouse)
  - You will need to install
    [reWASD](https://www.rewasd.com/), set your keyboard and/or mouse to emulate
    a virtual gamepad, then manually bind every button.
- GameCube Controllers
  - You will need to install
    [Delfinovin](https://github.com/Struggleton/Delfinovin)
    and calibrate your controller(s).