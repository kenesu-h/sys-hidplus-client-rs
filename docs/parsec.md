# Overview
In advance, here's some terminology:
- "Hosts" are those with hacked Switches that have sys-hidplus installed and can
  host game sessions.
- "Clients" are those who are connect to the host and play off their Switch.

When sys-hidplus is combined with a way to capture a Switch's video output and a
game streaming application such as Parsec, it can allow for remote play from
multiple places at once. In addition to allowing the use of normally-unsupported
controllers, this has a few advantages:
- As long as the host owns a game and its DLCs, clients have access to them too
  and can play with other clients. No one but the host has to own a Switch or an
  online subscription.
  - You could use this as a host to share your entire Switch library.
- Multiple people can remotely play on a Switch from anywhere, so as long as
  they have a stable internet connection with the host and vice-versa.
- Everyone has access to features of games that would normally only be
  available in local multiplayer.
- Everyone can play multiplayer Switch games at potentially lower input delays
  than if you were to play on Nintendo Online. 
- Everyone has access to all the benefits of a hacked Nintendo Switch,
  including game mods and tools for save data backups.

However, this also has some disadvantages:
- The host needs a hacked Switch, which may not be an option for those with
  newer Switches, or may be an uncomfortable experience for those who aren't
  willing to dive in and don't want to be banned.
  - Hacking and using sysmodules shouldn't guarantee a ban. People have used
    Smash mods and sysmodules online, and yet still aren't banned. However,
    doing such things always carries a risk.
- Both hosts and clients need solid and reliable internet connections to each
  other for a decent experience.
  - Game streaming isn't a silver bullet to replace games' bad netcode. Input
    delay can still be bad if the connection between the two isn't good, and
    unlike delay-based netcode, games will continue even if one player is
    lagging badly.

If you're still willing to continue with setup despite these disadvantages, read
on.

This documentation will tell you how to set this functionality up with
Parsec. Other game streaming applications like Moonlight can be used, but for
the sake of simplicity and support for more than one client, this will only
cover Parsec.


# Parsec Hosts

## The host needs:
- A low-latency way to capture a Switch's video output. This includes:
  - A capture card:
    - Any one of Elgato HD60 S, S+, Pro, or Pro+ is **strongly recommended**.
    - Other brands work so as long as the latency is as low as possible.
  - [sys-dvr](https://github.com/exelix11/SysDVR)
- An application to view a Switch's captured video output: 
  - [OBS](https://obsproject.com/)
  - [Elgato Game Capture HD](https://www.elgato.com/en/downloads) if you're
    using an Elgato capture card.
  - [The sys-dvr wiki](https://github.com/exelix11/SysDVR/wiki)
    is best at explaining what you need to do if you're using sys-dvr.
- A Nintendo Switch with sys-hidplus installed.
  - Consult [the Installation & Usage page](./usage.md) for more information on
    this.
- A computer with:
  - sys-hidplus-client-rs downloaded and setup.
    - Again, consult [the Installation & Usage page](./usage.md) for more
      information on this.
  - [Parsec](https://parsec.app/) installed.
- A solid, reliable internet connection.
  - An ethernet connection is **strongly recommended**. Your clients may suffer
    stream lag and input delay if you don't have one.

## Directions
1. If you haven't already, launch Parsec and the application displaying your
   Switch's video output.
2. Launch the client, connect it to your Switch, and keep it open.
3. In Parsec, go to the Arcade and start hosting the application displaying your
   Switch's video output.
  - If you're using Elgato Game Capture HD, you may have to make the window
    fullscreen for it to be properly hosted.
4. Send the resulting link to all your (to-be) Parsec clients.
5. Once they've connected to your session, they can connect controllers as you, 
   the host, usually would (press the right bumper).
6. Have fun!

**You must keep your Switch's video output focused in order for your clients'
inputs to go through.**

Feel free to connect your controller(s) through the Switch or your computer, as
needed. Your clients' controllers will all be disconnected either once they
leave your session, or you close it yourself.


# Parsec Clients
"Clients" are those who connect to a Parsec host.

## The client needs:
- A computer with [Parsec](https://parsec.app/) installed.
- At least one controller.
- A solid, reliable internet connection.
  - An ethernet connection is **strongly recommended**. You may encounter stream
    lag and input delay if you don't have one.

If you're using a Switch Pro Controller, you may have to have
[Steam](https://store.steampowered.com/) open with Switch Pro Controller
configuration enabled.

## First-Time Setup
It's recommended that clients change these Parsec settings to help minimize
input delay:
- VSync should be set to "Off".
- Decoder should be set to hardware. This tends to be every option that isn't
  "Software". If you have multiple options, try picking the one that corresponds
  with your dedicated and/or strongest GPU.

## Directions
1. Connect to your host using the link they should've sent you.
2. Plug in your controller(s), press the right bumper, and have fun!

**You must keep the host's stream focused in order for your inputs to go
through.**

No need to do anything special on your end. Just disconnect from Parsec whenever
you're done playing with the host, or wait for them to end the session.


# Common Problems

## Stream Lag and Input Delay
You may encounter stream lag or input delay if you're playing as a client. This
is usually because of an unstable and/or low-speed connection, or if you're far
from the host in terms of geographic location.

There are ultimately a ton of factors that can contribute to this and it is
difficult to diagnose the cause, but assuming that the host's Switch and PC are
on the same network, it's generally the case that neither the sysmodule nor
client are responsible for input delay. Try every possible way to improve your
connection; you and the host can try closing background processes on your
computers, or changing to an ethernet connection if you're on
wi-fi.

If nothing improves the connection, you unfortunately might just have to deal
with it or call it quits. To my knowledge, there's not much more that can be
done other than moving closer to the host or having generally higher internet
speeds, which obviously aren't the most feasible things for everyone.

If anyone has any other potential ways to improve this, please let me know.

## Can't connect to the host.
There are also a ton of factors that can contribute to this, but one of the most
common reasons for this can be if either the client or the host are behind a
strict firewall. School networks tend to have this, and will prevent clients
from connecting to a host and vice-versa.

Luckily, you have potential ways to get around this:
- The person on the network in question can use a VPN to circumvent the firewall,
  but this will likely add input delay.
- It also might be possible for the host to attempt
  [port-forwarding for Parsec](https://support.parsec.app/hc/en-us/articles/360003146251-Starting-Port-On-The-Hosting-Computer)
  but I personally haven't had any success with this. It may also be difficult
  to convince school network administrators to forward ports.
- I have, however, had success with using
  [Moonlight](https://moonlight-stream.org/) to host. While this requires the
  host to have an NVIDIA GPU and limits the amount of connections to just one,
  it is a confirmed way to circumvent this issue. Maybe you could use Moonlight
  for a client behind a school network and use Parsec for everyone else,
  although this might be on the demanding side in terms of bandwidth and GPU
  usage.