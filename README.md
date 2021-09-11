# sys-hidplus-client-rs
This is a client for the Nintendo Switch homebrew sysmodule "sys-hidplus",
written in Rust. sys-hidplus is not my project and is developed by
PaskaPanishkes. You can find its repo at: 
[PaskaPanishkes/sys-hidplus](https://github.com/PaskaPinishkes/sys-hidplus)

# Context
sys-hidplus is a sysmodule for a Nintendo Switch hacked to run custom firmware.
A sysmodule can be seen as a background service like those on most computers.
This sysmodule allows the Switch to receive controller inputs sent by a computer
using a client. In other words, controllers can be used for a Switch over a
network rather than through a direct USB or Bluetooth connection. This in turn
allows users to use normally unsupported controllers - such as Xbox and PS4
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
Pask has also
[rewritten their original client using C#](https://github.com/PaskaPinishkes/SwitchSysHidplusClient),
but they suspect that it may only work for Windows. For the time being, I plan
to continue working on this client to support Mac and Linux computers, and as a
learning experience for myself. Most of all however, I want to use this as an
outlet to implement and test more experimental features, which Pask could add
to their client at a later point once we find that they're stable. **I want to
stress that as a result, this is NOT meant to be a replacement for Pask's work,
but rather an alternative for those who want to use these experimental features
and don't mind the possibility of something breaking.**

## What happens when something breaks? How often?
I know I made it sound scary, but there shouldn't be anything that could do
permanent damage to your computer or your Switch. While there is a possibility
that something bad could happen, I didn't come across any fatal problems after
over a month of testing and on-and-off usage, not even when crashes happened.
Even then, there was a time where the client crashed and the last input I made
hung - in other words, I was holding A as the client crashed and the sysmodule
continued to act as if I were still holding A. This was just fixed by
restarting and reconnecting the client though. I've also never seen the
sysmodule or my Switch crash, but you could just uninstall the sysmodule if that
ever happened.

The client also only ever crashed when I made a major coding oversight, or if a
library I used did the same. However, these are somewhat rare and the crashes
that do happen because of libraries can be caught and stopped. Either way,
crashes shouldn't happen very often at all and I'm told the client is quite
stable, though this may change as new features are being added.

If you aren't deterred by any of that, read onto the next few sections for
details about new features, installation, usage, and setup with Parsec:
 
## New Features
As of 1.1.0-alpha.4, here are the new features that this client has:
- Support for up to 8 controllers, allowing you to take advantage of Smash's
  8-player mode.
- The home button is usable by controllers.
- You can configure these settings for individual controller slots:
  - Artificial input delay, useful for putting yourself at an even playing
    field with those who have some input delay playing through Parsec.
  - Left and right analog stick deadzones, useful for eliminating controller
    drift and helping stick flicks get recognized more easily.
- These settings are saved to a file and loaded for every client session.
- Users press a button (right bumper) to assign their controller to the first
  open slot. This is a QOL change and is intended to give users more control
  over which controller goes into what slot.

## Potential Problems
These are potential problems my forked sysmodule and client may have. It would
be appreciated if people could test these scenarios and report any appropriate
information.
- Using the GUI may increase input delay.
  - The command line interface, however, is available for use as an alternative
    and will likely have minimal input delay in comparison.
- Input delay may increase with an increased amount of players.
  - I have only tested with with four concurrent players at most, but there was
    no noticeable input delay. However, this may not be the same case for more
    than four players, as well as hosts with weaker computers.

# Installation & Usage
For information on this, please consult
[the Installation & Usage page](./docs/usage.md).

# Parsec Usage
If you want to setup sys-hidplus to allow for remote play of your Switch, please
consult [the Parsec page](./docs/parsec.md).

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
- Kemosahbee for testing the client, reporting bugs, and giving suggestions as
  well as feedback. Many important bugs were found thanks to him and his testing
  efforts.
- cathery for their sysmodule sys-con, which helped me figure out how to emulate
  the home button.
