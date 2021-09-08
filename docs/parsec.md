# Overview
When sys-hidplus is combined with a game streaming application such as Parsec,
it can allow for remote play of a single Nintendo Switch from multiple places at
once. This documentation will tell you how to set this functionality up with
Parsec. Other game streaming applications like Moonlight can be used, but for
the sake of simplicity and support for more than one client, this will only
cover Parsec.


# Parsec Hosts
In this case, the "Host" is the person with sys-hidplus installed on their
Switch, and the client downloaded onto their computer. Consult
[the Installation & Usage page](./usage.md) for information on this.

## The host needs:
- A capture card with low latency:
  - Any one of Elgato HD60 S, S+, Pro, or Pro+ is **strongly recommended**.
  - Other brands work so as long as the latency is as low as possible.
- An application to view capture card input:
  - [Elgato Game Capture HD](https://www.elgato.com/en/downloads)
  - [OBS](https://obsproject.com/)
- A Nintendo Switch with sys-hidplus installed.
- A computer with:
  - sys-hidplus-client-rs downloaded and setup.
  - Parsec installed.
- A solid, reliable internet connection.
  - An ethernet connection is **strongly recommended**. Your clients may suffer
    stream lag and input delay if you don't have one.

## Directions
1. If you haven't already, launch Parsec and the application displaying your
   capture card input.
2. Launch the client, connect it to your Switch, and keep it open.
3. In Parsec, go to the Arcade and start hosting the application displaying your
   capture card input.
  - If you're using Elgato Game Capture HD, you may have to make the window
    fullscreen for it to be properly hosted.
4. Send the resulting link to all your (to-be) Parsec clients.
5. Once they've connected to your session, they can connect controllers as you, 
   the host, usually would (press the right bumper).
6. Have fun!

Feel free to connect your controller(s) through the Switch or your computer, as
needed. Your clients' controllers will all be disconnected once they leave your session.


# Parsec Clients
"Clients" are those who connect to a Parsec host.

## The client needs:
- A computer with Parsec installed.
- At least one controller.
- A solid, reliable internet connection.
  - An ethernet connection is **strongly recommended**. You may encounter stream
    lag and input delay if you don't have one.

## First-Time Setup
It's recommended that clients change these Parsec settings to help minimize
input lag:
- VSync should be set to "Off".
- Decoder should be set to hardware. This tends to be every option that isn't
  "Software".

## Directions
1. Connect to your host using the link they should've sent you.
2. Plug in your controller(s), press the right bumper, and have fun!

No need to do anything special on your end. Just disconnect from Parsec whenever
you're done playing with the host, or wait for them to end the session.


# Common Problems

## Stream Lag and Input Delay
You may encounter stream lag or input delay if you're playing as a client. This
is usually because of an unstable and/or low-speed connection, or if you're far
from the host in terms of location.

There are ultimately a ton of factors that can contribute to this and it is
difficult to diagnose the cause, but assuming that the host's Switch and PC are
on the same network, it's generally the case that neither the sysmodule nor
client are responsible for input delay. If possible, do everything possible to
improve your connection; you and the host can try closing background processes
on your computers, or changing to an ethernet connection (if possible) if you're
on wi-fi.

## Can't connect to the host.
There are also a ton of factors that can contribute to this, but one of the most
common reasons for this can be if either the client or the host are behind a
strict firewall. University networks tend to have this, and will prevent clients
from connecting to a host and vice-versa. The person on the network in question
can use a VPN to circumvent the firewall, but this will likely add input delay.