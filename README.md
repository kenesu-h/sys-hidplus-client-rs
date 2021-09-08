# sys-hidplus-client-rs
This is a client for the Nintendo Switch homebrew sysmodule "sys-hidplus",
written in Rust. sys-hidplus is not my project and is developed by
PaskaPanishkes. You can find its repo at: 
[PaskaPanishkes/sys-hidplus](https://github.com/PaskaPinishkes/sys-hidplus)

# Context
sys-hidplus is a sysmodule for a Nintendo Switch hacked to run custom firmware.
A sysmodule can be seen as a background service like those on most computers.
This sysmodule allows the Switch to receive controller inputs sent by a computer
using a client. In other words, gamepads can be used for a Switch over a network
rather than through a direct USB or Bluetooth connection. This in turn enables
users to use normally unsupported controllers - such as Xbox and PS4
controllers, or even keyboards - although similar functionality is provided by:
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
written in Python, but I decided to rewrite it in Rust and take the opportunity
to add some new features. Pask is aware of this client and is fine with its
continued development.

# Foreword
While Pask has also
[rewritten their original client using C#](https://github.com/PaskaPinishkes/SwitchSysHidplusClient),
they suspect it may only work for Windows. I intend to continue working on this
client to support Macs and Linux machines. I also want to use this opportunity
as a learning experience and an outlet to implement more experimental
functionality. As such, keep in mind that this client will likely have
different features from Pask's client, which may or may not be
cross-platform-friendly.

To bounce ideas off each other - and for your own convenience - here's the
differences between our clients (that come to mind):
- Is a command line application. This is nowhere as easy to use compared to a
  GUI, which is in development but might not complete for a while.
- Settings are done from a single configuration file, but can be edited through
  the client itself.
- Users press a button (right bumper) to assign their controller to a slot. This
  is a QOL change and is intended to give users more control over slot mappings.
- Artificial input delay can be added to individual controller slots.
- Supports up to 8 controllers through SDL.
- The home button can be used by emulated controllers.

Here's some of the differences that Pask's client has to my knowledge:
- Is not command line-based; uses a GUI.
- Controllers are automatically assigned when connected and disconnected.
- Has anarchy mode, where everyone uses the same controller.

# Installation & Usage
For information on this, please consult
[the Installation & Usage page](./docs/usage.md).

# Build from Source
For information on this, please consult [the Build page](./docs/build.md).

# Known Issues
- Sideways Joy-Con support is limited.
  [This is out of our control for the most part.](https://github.com/switchbrew/libnx/issues/567)
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
- The command line interface will fail to recognize commands if you use arrow
  keys. I'm personally not sure how to fix this, but for now, try to avoid using
  any arrow keys.

# Contact
If you want to contact me, you can reach me at:
- officialkennyho@gmail.com
- Kenesu#2586 on Discord
- kenesu_h on GBATemp

I tend to respond more quickly to Discord messages than any other form of
communication.

# Credits
Special thanks go to:
- PaskaPinishes for sys-hidplus and their clients, as well as everyone who
  helped them out. I literally would not be working on this project if it
  weren't for them.
- Kemosahbee for testing the client and reporting bugs. Many critical bugs were
  found thanks to him and his testing efforts.
- cathery for their sysmodule sys-con, which helped me figure out how to emulate
  the home button.
