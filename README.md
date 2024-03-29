# sys-hidplus-client-rs

## (9/20/23) Note to anyone who takes a look at this repo:

imo this codebase is just a plain mess and you're probably gonna hate looking at it as much as I do lol (hence why I haven't worked on it at all recently)

I'm planning a rewrite (at some point) but have no clue when I'll be _motivated_ to do it at all

until then, if you really want to take a look, be sure to take a look at branch 1.1.0-alpha. even though I never merged it to main, it'll have the latest changes with the following off the top of my head:
- 8 player support
- anarchy mode

just so you don't have to scroll all the way down, I'm `kenesu_h` on Discord and GBATemp, but I am way more likely to quickly respond to you on Discord

you won't bother me too much, if you want any insight into the codebase, just give a DM and point out that you're not just some rando/scammer

for everyone who's starred this, sorry for the non-eventful commit, but I've noticed more people have been doing so lately and thought that folks should at least be aware of what's been going on

## Overview

This is a client for the Nintendo Switch homebrew sysmodule "sys-hidplus",
written in Rust. sys-hidplus is not my project and is developed by
PaskaPanishkes. You can find its repo at: 
[PaskaPanishkes/sys-hidplus](https://github.com/PaskaPinishkes/sys-hidplus)

# Context
sys-hidplus is a sysmodule for a Nintendo Switch hacked to run custom firmware.
This sysmodule allows the Switch to receive controller inputs sent by a computer
using a client. This in turn enables users to use normally unsupported
controllers - such as an Xbox or PS4 controller. This can also be used alongside
a remote game streaming service such as Parsec to allow people play Switch games
from practically anywhere.

This premise is what attracted me to help contribute to Pask's work, so I ended
up wanting to rewrite and refactor their original input client, written in
Python. Pask is aware of this client and is fine with its development.

# Foreword
While Pask has also rewritten their original client using C# - which features a
GUI unlike mine, 
[check it out](https://github.com/PaskaPinishkes/SwitchSysHidplusClient)! -
they suspect it may only work for Windows. I intend to continue working on this
client to support Macs and Linux machines. I also want to use this opportunity
as a learning experience and an outlet to implement more experimental
functionality.  As such, keep in mind that this client will likely have
different features from Pask's client, which may or may not be
cross-platform-friendly.

For both Pask and I's convenience - so we can bounce ideas off each other -
here's some of the differences my client has (that come to mind):
- Is a command line application.
- Settings are done from a single configuration file, but can be edited even
  through the client itself.
- Users press a button (right bumper) to assign their controller to a slot.
- Controllers are cleaned up and disconnected from the Switch when the client is
  stopped.
- Uses SDL as an input library that supports 4+ controllers.
  - Keep in mind that this client cannot actually emulate 4+ controllers until
    sys-hidplus is updated to do the same. It's just that the framework for it
    is there.

Here's some of the differences that Pask's client has to my knowledge:
- Is not command line-based; uses a GUI.
- Controllers are automatically assigned when connected and disconnected.
- Has anarchy mode, where everyone uses the same controller.

# Download and Usage
Download the latest version of the client from
[the releases page](https://github.com/kenesu-h/sys-hidplus-client-rs/releases).
Extract the archive's files into a folder using 7zip or some equivalent - WinRAR
works. Preferably, the folder should be dedicated to the client.

This client supports Windows, Mac OS X, and GNU/Linux, although support for the
latter two have not been tested since I lack machines for both. You may have to
consult the **Compiling** section if you are using either of them, as I only have a
Windows machine with which to build executables.

This is a currently intended to be a command line application and as such, it is
recommended that you have a terminal or shell on hand to execute the following
commands.

## Running the Client
Open your terminal and navigate to the directory containing your executable.

### Windows - cmd
If you're using `cmd` to run the client, run the following:
```
client-rs.exe
```

### Windows - PowerShell (or other Unix-like shells),
If you're using PowerShell (or other Unix-like shells) - such as cygwin - run
the following:
```
./client-rs.exe
```

### Mac OS X and Linux
Open your OS's terminal/shell and run the following:
```
./client-rs
```

Either method should give you a welcome message. You'll find that typing `start`
will notify you that you'll have to set a server IP first. You can set it by
using the command `set_server_ip 'server_ip'`, where 'server_ip' is replaced
with the IP of the Switch you want to connect to. You can also edit
`config.toml`, which is generated within your current directory, but you can
generally edit all its fields through commands.

After you've set your Switch's IP, you can type `start` again to start the
client. You'll find that the client should be ready to connect controllers to
your Switch.

## Additional Configuration
The client offers additional configuration, such as changing a slot's controller
type and input delay - input delay in particular is helpful for giving a host
lag to match their clients.

You can change a slot's controller type using `set_switch_pad 'i' 'switch_pad'`.
'i' represents the "index" of the slot you want to change, and 'switch_pad' is
any one of the following:
```
Disconnected
ProController
JoyConLSide
JoyConRSide
```
The "index" is just the slot # - 1. For example, if you wanted to change a
controller in slot 2 to a sideways left JoyCon, run
`set_switch_pad 1 JoyConLSide`.
If you run this while the client is running, you may have to restart it (you can
type `restart`) for the changes to take effect. For more information on this,
you can use `help set_switch_pad`.

In a similar way, you can change a slot's input delay using
`set_input_delay 'i' 'input_delay'`.
'input_delay' must be a positive number. Unlike `set_switch_pad` though, you
don't need to restart the client for the changes to work. For more information
on this, you can use `help set_input_delay`.

Alternatively, you can edit `config.toml` too. Just be sure to follow the
existing format.

## Connecting Controllers
Plug into your computer the controllers you want to use on your Switch. If
you're want to use a GameCube controller via an adapter, please
[install Delfinovin first](https://github.com/Struggleton/Delfinovin).

When you want to activate a controller, press your right bumper:
- On Xbox controllers, this will be RB.
- On PlayStation controllers, this will be R1.
- On Switch controllers, this will be R.
- On GameCube controllers, this will be Z assuming you have Delfinovin setup.
- On any other controllers, you get the point.

Activating a controller will assign it to the first available slot, and will not
override slots that are already connected. You may have to make a few button
inputs for your controller to be recognized by your Switch.

Repeat this process for every controller you want to connect.

## Disconnecting Controllers
Disconnecting controllers is as easily as unplugging them from your computer.
The slot they occupied will become open and usable by other controllers. You can
reconnect your controller and reactivate it at any time.

## Closing the Client
**This is a pretty important step if you want to cleanly disconnect your
controllers**. You can run `stop` if you want to stop the client without closing
it. You should see a message where the client is cleaning up the controllers.
After about 3 seconds, you should see another message telling you that the
cleanup's done. You'll have to run `exit` to completely close the client.
Alternatively, you can run `exit` while the client is still running to stop then
close the client. Feel free to close your terminal/shell after this point.

# Compiling
This section assumes that you have Rust installed on your computer, preferably
using [rustup](https://rustup.rs/).

Compiling should be as simple as navigating to this repo's directory and running
`cargo build`. You should have an executable for your operating system in
`target/debug`.

Alternatively, you can run (and build) by running `cargo run`. Running the
client otherwise follows the same steps as **Download and Usage**, minus the
download part of course.

# Known Issues
- sys-hidplus is known to have input delay in demanding games. I don't have any
  games to personally test this with, but initial obstacles in development seem
  to support this; namely, input delay was present when inputs were sent at a
  much higher frequency than the frame rate.
- If a controller is forcibly disconnected by the Switch - like through the
  "Change Grip/Order" menu or the "Disconnect" button in Smash Ultimate - **you
  cannot reconnect it until you restart your Switch**. I'm also told that you
  can restart Parsec (if any of the affected controllers were through Parsec),
  but I'm not totally sure. Either way, it's possible a future sys-hidplus 
  update can resolve this, but until then, try to avoid any menus and options
  that forcibly disconnect controllers (if you can).
- As of an unofficial build for sys-hidplus (for 12.x support), sideways Joy-Con
  emulation doesn't seem to work properly. That said, this has yet to be tested
  in games that use sideways Joy-Cons, so feel free to try it out.
- Using arrow keys in the command line interface messes up the next few
  commands. Not entirely sure how to fix this. For now, try to avoid using any
  arrow keys and if you do, try to move your cursor all the way to the last
  line.

# Contact
If you want to contact me, you can reach me at kenesu_h on Discord (preferred) or GBATemp.

# Credits
Credits go to PaskaPinishes for sys-hidplus (and the associated client) as a
whole, as well as everyone who helped them out. Without them, I literally would
not be working on this project.
