# sys-hidplus-client-rs
This is a client for the Nintendo Switch homebrew sysmodule "sys-hidplus",
written in Rust. sys-hidplus is not my project and is developed by
PaskaPanishkes. You can find its repo at: 
[PaskaPanishkes/sys-hidplus](https://github.com/PaskaPinishkes/sys-hidplus)

# Table of Contents
<ol>
  <li><a href="#context">Context</a></li>
  <li><a href="#foreword">Foreword</a></li>
  <li><a href="#new-features">New Features</a></li>
  <li>
    <a href="#i-want-to">I want to:</a>
    <ol>
      <li>
        <a href="#setup-the-client-for-general-use">
          Setup the client for general use.
        </a>
      </li>
      <li>
        <a href="#setup-the-client-for-remote-play">
          Setup the client for remote play.
        </a>
      </li>
      <li>
        <a href="#report-an-issue-or-request-a-feature">
          Report an issue or request a feature.
        </a>
        <ol>
          <li><a href="#for-issues">For Issues</a></li>
          <li><a href="#for-feature-requests">For Feature Requests</a></li>
        </ol>
      </li>
      <li>
        <a href="#work-with-the-source-code">
          Work with the source code.
        </a>
      </li>
      <li>
        <a href="#contribute-to-this-project">
          Contribute to this project.
        </a>
      </li>
    </ol>
  </li>
  <li><a href="#contact-me">Contact Me</a></li>
  <li><a href="#credits">Credits</a></li>
</ol>

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
but they suspect that it may only work for Windows. I plan to continue working
on this client to support Mac and Linux computers, and as a learning experience
for myself. Most of all however, I want to use this as an outlet to implement
and test more experimental features that Pask could add to their client at a
later point. **I want to stress that as a result, this is NOT meant to be a
replacement for Pask's work, but rather an alternative for those who want to use
these experimental features and don't mind the possibility of something
breaking.** The worst case scenario just involves crashes or input lag, which
shouldn't happen often at all. I'm also told the client is quite stable, though
this may change as new features are added.

If you aren't deterred by any of that, read onto the next few sections for
details about new features, setup, and more.
 
# New Features
As of the latest release, here are the new features that this client has:
- Support for up to 8 controllers.
- The home button is usable by controllers.
- You can configure these settings for individual controller slots:
  - Artificial input delay.
  - Left and right analog stick deadzones.
- Persistent settings between client sessions.
- Controller assignment to slots by pressing a button (right bumper).

# I want to:
## Setup the client for general use.
Head over to [Setup](./docs/setup.md) and follow the instructions there.

## Setup the client for remote play.
Head over to [Remote Play](./docs/remote-play.md) and follow the instructions there.

## Report an issue or request a feature.
You can
[open a GitHub issue](https://github.com/kenesu-h/sys-hidplus-client-rs/issues)
with details about your issue or feature request. Before you do though, I ask
that you double-check the open and [known issues](./docs/issues.md) before you open
one for yourself.

There isn't a strict format you have to follow, but doing the following will help
me debug your issue better or get a better idea of what you want out for a
feature:

### For Issues
Describe what the issue is and give detailed steps on how to recreate the
situation, if possible.

### For Feature Requests
Describe what the feature you're requesting and how you plan to use it - in other
words, your use case for it.
[This issue](https://github.com/kenesu-h/sys-hidplus-client-rs/issues/3) is a
pretty good example of how to write one.

## Work with the source code.
By all means, of course. I welcome anyone to make changes to this project as they
see fit, so as long as you credit myself, Pask, and anyone else who contributed.
You are also legally obligated to abide by
[the GNU GPL-3 license](https://tldrlegal.com/license/gnu-general-public-license-v3-(gpl-3)).

Head over to [Build](./docs/build.md) for information on building this project from
source.

## Contribute to this project.
Likewise, if you have any work you'd like to contribute to the project, feel free
to [open a pull request](https://github.com/kenesu-h/sys-hidplus-client-rs/pulls)
and/or contact me directly. I'm still new to working with people on GitHub, but
I won't bite.

# Contact Me
You can reach me at:
- Kenesu#2586 on Discord
- kenesu_h on GBATemp

I tend to respond more quickly to Discord messages than any other form of
communication, but I check all of them pretty frequently.

# Credits
Special thanks go to:
- PaskaPinishes for sys-hidplus and their clients, as well as everyone who
  helped them out.
- Kemosahbee for testing the client, reporting bugs, chipping in with 
  suggestions, and giving feedback.
- cathery for their sysmodule sys-con, which helped me figure out how to emulate
  the home button.
- ConspiracyFactualist on GBATemp for reporting an input-related bug and being
  patient with testing.