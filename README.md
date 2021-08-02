# sys-hidplus-client-rs
This is a client for the Nintendo Switch homebrew sysmodule "sys-hidplus",
written in Rust. sys-hidplus is not my project and is developed by
PaskaPanishkes. You can find its repo at: 
[PaskaPanishkes/sys-hidplus](https://github.com/PaskaPinishkes/sys-hidplus)

# Context
sys-hidplus is a sysmodule for a Nintendo Switch hacked to run custom firmware.
This sysmodule allows the Switch to receive controller inputs sent by a computer
using a client. This in turn enables users to use normally unsupported
controllers - such as an Xbox or PS4 controller. This can be used alongside a
remote game streaming service such as Parsec to allow people play Switch games
from practically anywhere.

This premise is what attracted me to help contribute to Pask's work, so I ended
up wanting to rewrite and refactor their original input client, written in
Python.

# Foreword
While Pask has also rewritten their original client using C# - which features a
GUI unlike mine, 
[check it out](https://github.com/PaskaPinishkes/SwitchSysHidplusClient)! -
they suspect it may only work for Windows. I intend to continue working on this
client to support Macs and Linux machines. I also want to use this opportunity
as a learning experience and an outlet to implement more experimental functions.
As such, keep in mind that this client may have different features from Pask's
client, which may or may not be cross-platform-friendly.

For both Pask and I's convenience - so we can bounce ideas off each other -
here's some of the differences my client has (that come to mind):
- Is a command line application.
- Settings are done from a single configuration file.
- Users press a button (right bumper) to assign their controller to a slot.
- Controllers are cleaned up and disconnected from the Switch when the client is
  closed.
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

### Windows - PowerShell (or other Unix-like shells), Mac, Linux
If you're using PowerShell (or other Unix-like shells), Mac, or Linux, run the
following:
```
client-rs
```

Either method should bring up a message and generate a file called `config.toml`
within your current directory. Open it using your favorite text editor. You
should see something like this:
```
server_ip = ''
switch_pad_1 = 'ProController'
switch_pad_2 = 'ProController'
switch_pad_3 = 'ProController'
switch_pad_4 = 'ProController'
...
```
Between the single quotes next to `server_ip`, type out your Switch's IP
address. For example, if its IP were 192.168.1.199, the `server_ip` line should
look like this:
```
server_ip = '192.168.1.199'
```
Rerun the client, and you'll find that there should be a "Ready" message this
time. If the client hasn't exited at this point, this means your client is ready
to connect controllers to your Switch.

## Additional Configuration
You can change any of the `switch_pad_x` values (where `x` is a number) to any
one of the following:
```
ProController
JoyConLSide
JoyConRSide
```
Be sure to surround the value in single quotes though, just like the original
configuration.

## Connecting Controllers
Plug into your computer the controllers you want to use on your Switch. If
you're want to use a GameCube controller via an adapter, please
[install Delfinovin first](https://github.com/Struggleton/Delfinovin).

When you want to activate a controller, press your right bumper:
- On Xbox controllers, this will be RB.
- On PlayStation controllers, this will be R1.
- On Switch controllers, this will be R.
- On GameCube controllers, this will be Z assuming you have Delfinovin setup.

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
controllers**. Focus or click on the terminal/shell with your client open. Press
Ctrl + C (on Mac this might be Command + C). You should see a message where the
client is cleaning up the controllers. After about 3 seconds, you should see
another message telling you that the cleanup's done. Feel free to close your
terminal/shell after this point.

# Compiling
This section assumes that you have Rust installed on your computer, preferably
using [rustup](https://rustup.rs/).

Compiling should be as simple as navigating to this repo's directory and running
`cargo build`. You should have an executable for your operating system in
`target/debug`.

Alternatively, you can run (and build) by running `cargo run`. Running the
client otherwise follows the same steps as **Download and Usage**.

# Known Issues
- If a controller is forcibly disconnected by the Switch - like through the
  "Change Grip/Order" menu or the "Disconnect" button in Smash Ultimate - **you
  cannot reconnect it until you restart your Switch**. It's possible a future
  sys-hidplus update can resolve this, but until then, try to avoid any menus
  and options that forcibly disconnect controllers (if possible).
- As of an unofficial build for sys-hidplus (for 12.x support), sideways Joy-Con
  emulation doesn't seem to work properly. That said, this has yet to be tested
  in games that use sideways Joy-Cons, so feel free to try it out.

# Contact
If you want to contact me, you can reach me at Kenesu#2586 on Discord.

# Credits
Credits go to PaskaPinishes for sys-hidplus (and the associated client) as a
whole, as well as everyone who helped them out. Without them, I literally would
not be working on this project.
