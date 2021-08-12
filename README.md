# sys-hidplus-client-rs
This is a client for the Nintendo Switch homebrew sysmodule "sys-hidplus",
written in Rust. sys-hidplus is not my project and is developed by
PaskaPanishkes. You can find its repo at: 
[PaskaPanishkes/sys-hidplus](https://github.com/PaskaPinishkes/sys-hidplus)

# Context
sys-hidplus is a sysmodule for a Nintendo Switch hacked to run custom firmware.
This sysmodule allows the Switch to receive controller inputs sent by a computer
using a client. In other words, the Switch can be controlled over a network
rather than through a direct USB or Bluetooth connection. This in turn enables
users to use normally unsupported controllers - such as an Xbox or PS4
controller - although this same functionality is provided by:
- [sys-con](https://github.com/cathery/sys-con), which offers USB connectivity
  for most popular controllers.
- [MissionControl](https://github.com/ndeadly/MissionControl), which offers
  Bluetooth connectivity for a large number of wireless controllers.

However, since sys-hidplus accomplishes this using a network connection, it can
be used alongside a low-latency capture card and a remote play service to enable
Switch gameplay from practically anywhere, using any computer-compatible
controller.

This premise is what attracted me to help contribute to Pask's work, so I ended
up rewriting and refactoring their original input client. The input client was
written in Python, but I decided to rewrite it in Rust. Pask is aware of this
client and is fine with its development.

# Foreword
While Pask has also rewritten their original client using C# - which features a
GUI unlike mine, 
[check it out](https://github.com/PaskaPinishkes/SwitchSysHidplusClient)! -
they suspect it may only work for Windows. I intend to continue working on this
client to support Macs and Linux machines. I also want to use this opportunity
as a learning experience and an outlet to implement more experimental
functionality. As such, keep in mind that this client will likely have
different features from Pask's client, which may or may not be
cross-platform-friendly.

For both Pask and I's convenience - so we can bounce ideas off each other -
here's some of the differences my client has (that come to mind):
- Is a command line application. This is nowhere as easy to use compared to a
  GUI, and will likely be swapped out in the future.
- Settings are done from a single configuration file, but can be edited through
  the client itself.
- Users press a button (right bumper) to assign their controller to a slot. This
  is a QOL change and is intended to give users more control over slot mappings.
- Artificial input delay can be added to individual controller slots.
- Supports up to 8 controllers through SDL.

Here's some of the differences that Pask's client has to my knowledge:
- Is not command line-based; uses a GUI.
- Controllers are automatically assigned when connected and disconnected.
- Has anarchy mode, where everyone uses the same controller.

# Download and Usage
Download the latest version of the client from
[the releases page](https://github.com/kenesu-h/sys-hidplus-client-rs/releases).
Extract the archive's files into a folder using 7-Zip or some equivalent -
WinRAR works. Preferably, the folder should be dedicated to the client.

This client supports Windows, Mac OS X, and GNU/Linux, although support for the
latter two have not been tested since I currently lack machines for both. You
may have to consult the **Compiling and Running from Source** section if you are
using either of them, as I only have a Windows machine to build executables
from.

This is a currently intended to be a command line application and as such, it is
recommended that you have a terminal or shell on hand to execute the following
commands unless you're on Windows.

## Running the Client

### Windows
You can just run the executable directly and it should automatically open a
command prompt.

### Mac OS X and Linux
Open your OS's terminal/shell, navigate to the directory containing your
executable, and run the following:
```
./client-rs
```
If the above doesn't work, you may have to `chmod` it to make it executable:
```
chmod +x client-rs
```
Try to run the executable again after 'chmod'ing.

## Starting the Client
Either method should give you a welcome message. You'll find that typing `start`
will notify you that you'll have to set a server IP first. You can set it by
using the command `set_server_ip 'server_ip'`, where 'server_ip' is replaced
with the IP of the Switch you want to connect to.

You can also edit `config.toml`, which is generated within your current
directory, but you can generally edit all its fields through commands.

After you've set your Switch's IP, you can type `start` again to start the
client. You'll find that the client should be ready to connect controllers to
your Switch.

## Additional Configuration
The client offers additional configuration, such as changing a slot's controller
type and input delay - input delay in particular is helpful for giving a host
lag to match their clients.

### Controller Types
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

A few warnings though:
- If you run this while the client is running, you may have to restart it (you
  can type `restart`) for the changes to take effect. For more information on
  this, you can use `help set_switch_pad`.
- BUG: Using either `JoyConLSide` or `JoyConRSide` causes the controller to be
  connected as an incomplete pair of Joy-Cons. I don't have a game to test with
  that may automatically connect these as sideways Joy-Cons, but use these
  settings at your own risk.

### Input Delay
In a similar way, you can change add artificial input delay to a controller slot
using `set_input_delay 'i' 'input_delay'`.

'input_delay' must be a positive number. Unlike `set_switch_pad` though, you
don't need to restart the client for the changes to work. For more information
on this, you can use `help set_input_delay`.

Alternatively, you can edit all this `config.toml` too. Just be sure to follow
the existing format.

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

## Stopping and Closing the Client
**This is a pretty important step if you want to cleanly disconnect all
connected controllers**. You can run `stop` if you want to stop the client
without closing it. You should see a message where the client is cleaning up the
controllers.  After about 3 seconds, you should see another message telling you
that the cleanup's done. You'll have to run `exit` to completely close the
client.  Alternatively, you can run `exit` while the client is still running to
stop then close the client. Feel free to close your terminal/shell after this
point.

# Compiling and Running from Source
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
If you want to contact me, you can reach me at Kenesu#2586 on Discord or
kenesu_h on GBATemp.

# Credits
Special thanks go to:
- PaskaPinishes for sys-hidplus and their clients, as well as everyone who
  helped them out. Without them, I literally would not be working on this
  project!
- Kemosahbee on GBATemp for testing the client, its 8-player support, and
  reporting related info - including bugs!
