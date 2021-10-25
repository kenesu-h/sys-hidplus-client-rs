# Remote Play Setup
This documentation is intended to help you through the steps of setting up
remote play from your Switch using Parsec, a remote desktop/play service. Other 
game streaming applications like Moonlight can be used, but for the sake of 
simplicity and support for more than one client, this will only cover Parsec.

# Table of Contents
<ol>
  <li>
    <a href="#foreword">Foreword</a>
    <ol>
      <li><a href="#terminology">Terminology</a></li>
      <li><a href="#pros-and-cons">Pros and Cons</a></li>
    </ol>
  </li>
  <li>
    <a href="#host-setup">Host Setup</a>
    <ol>
      <li><a href="#requirements">Requirements</a></li>
      <li><a href="#directions">Directions</a></li>
    </ol>
  </li>
  <li>
    <a href="#client-setup">Client Setup</a>
    <ol>
      <li><a href="#requirements-1">Requirements</a></li>
      <li><a href="#directions-1">Directions</a></li>
    </ol>
  </li>
  <li>
    <a href="#common-issues">Common Issues</a>
    <ol>
      <li>
        <a href="#stream-lag-and-input-delay">Stream Lag and Input Delay</a>
      </li>
      <li>
        <a href="#unable-to-connect-to-host">Unable to connect to host.</a>
      </li>
      <li>
        <a href="#clients-controller-doesnt-work">Client's controller doesn't work.</a>
      </li>
    </ol>
  </li>
</ol>

# Foreword
## Terminology
To help explain this, here's some terminology:
- "Hosts" are those with hacked Switches that have sys-hidplus installed and can
  host game sessions.
- "Clients" are those who are connect to the host and play off their Switch.

## Pros and Cons
While this is long-winded, it is recommended that you know about remote 
play's advantages and limitations before proceeding.
<table style="width: 100%">
  <tr>
    <th style="width: 50%">Pros</th>
    <th style="width: 50%">Cons</th>
  </tr>
  <tr>
    <td>
      <ul>
        <li>Only the host needs a Switch.</li>
        <li>Clients share access to the host's games and DLC.</li>
        <li>Multiple people can remotely play on a single Switch.</li>
        <li>Access to game features only available in local multiplayer.</li>
        <li>
          Potentially lower input delay than if you were to play on Nintendo 
          Online.
        </li>
        <li>
          Access to all the benefits of a hacked Switch, including game mods 
          and tools for save data backups (among other things).
        </li>
      </ul>
    </td>
    <td>
      <ul>
        <li>
          The host needs a
          <a href="https://nh-server.github.io/switch-guide/">hacked Switch</a>.
          <ul>
            <li>Newer Switches can't be hacked.</li>
            <li>
              Hacking a Switch isn't a comfortable experience for everyone.
            </li>
            <li>
              Although hacking and sysmodule usage don't guarantee bans, there's
              always a possibility that Nintendo may step in.
            </li>
          </ul>
        </li>
        <li>
          Hosts and clients need solid, reliable internet connections with each 
          other for a decent experience.
          <ul>
            <li>Input delay can still be present with poor connections.</li>
            <li>
              Game streaming isn't a silver bullet to replace games' bad 
              netcode.
            </li>
            <li>
              Distance and internet availability will make or break sessions.
            </li>
          </ul>
        </li>
      </ul>
    </td>
  </tr>
</table>

If you're still willing to continue with setup despite these cons, read on.

# Host Setup
If you are planning to host a session, follow these instructions to prepare
your Switch and computer for remote play.

## Requirements
- A low-latency Switch video capture method. This includes:
  - A capture card such as the Elgato HD60 S, S+, Pro, or Pro+.
  - [sys-dvr](https://github.com/exelix11/SysDVR)
- An application to view a Switch's captured video output: 
  - [OBS](https://obsproject.com/)
  - [The sys-dvr wiki](https://github.com/exelix11/SysDVR/wiki)
    is best at explaining what you need to do if you're using sys-dvr.
- A Nintendo Switch with [sys-hidplus](./setup.md#sysmodule) installed.
- A computer with:
  - [sys-hidplus-client-rs](./setup.md#client) installed.
  - [Parsec](https://parsec.app/) installed.
- A solid, reliable internet connection.
  - An ethernet connection is **strongly recommended**. Your clients may
    experience stream lag and input delay otherwise.

## Directions
1. Launch Parsec and the Switch video output application.
2. Launch the client, connect it to your Switch, and keep it open.
3. In Parsec, go to the Arcade and start hosting your video output application.
4. Send the resulting link to whoever you want to invite.
5. Once your clients have connected, they can connect controllers as you, the
   the host, usually would (press the right bumper).
6. Have fun!

**You must keep your Switch's video output focused in order for your clients'
inputs to be recognized.**

Feel free to connect your own controller(s) through the Switch or your 
computer as needed. Your clients' controllers will all be disconnected when
either they leave your session, or you close it yourself.


# Client Setup
If you're planning to connect to a host, follow these instructions to prepare
your computer for remote play.

## Requirements
- A computer with [Parsec](https://parsec.app/) installed.
- At least one controller.
- A solid, reliable internet connection.
  - An ethernet connection is **strongly recommended**. You may encounter stream
    lag and input delay otherwise.

If you're using a Switch Pro Controller, you may have to have
[Steam](https://store.steampowered.com/) open with Switch Pro Controller
configuration enabled in Big Picture.

## First-Time Setup 
It's recommended that clients change these Parsec settings to help minimize
input delay:
- VSync should be set to "Off".
- Decoder should be set to hardware.
  - This tends to be every option that isn't "Software".
  - If you have multiple options, try picking the one that corresponds with 
    your dedicated and/or strongest GPU.

## Directions
1. Connect to your host using the invite link they should've sent you.
2. Plug in your controller(s) and press the right bumper on the one(s) you want 
   to use.
3. Have fun!

**You must keep the host's stream focused in order for your inputs to be
recognized.**

No need to do anything special on your end. Just disconnect from Parsec whenever
you're done playing with the host, or wait for them to end the session.


# Common Issues
If you're experiencing issues at any point in remote play, be sure to read 
through this section first to make sure that they are connectivity issues or 
controller quirks, not issues with the sysmodule or client. If you believe that 
your issue is a sysmodule/client problem, feel free to
[open a GitHub issue](https://github.com/kenesu-h/sys-hidplus-client-rs/issues).

## Stream Lag and Input Delay
You may encounter stream lag or input delay if you're playing as a client. This
is usually because of an unstable and/or low-speed connection, or if you're far
from the host in terms of geographic location.

You can try several things:
- Make sure the host's Switch and PC are on the same network.
  - Connect both devices via to the network via ethernet.
- Both you and the host can try closing programs open in the background.
- If you're on wi-fi, change to an ethernet connection if possible.

If nothing improves the connection, you unfortunately might just have to deal
with it or call it quits. To my knowledge, there's not much more that can be
done other than moving closer to the host or having generally higher internet
speeds, which obviously aren't the most feasible things for everyone.

If anyone has any other potential ways to improve this, please let me know.

## Unable to connect to host.
One of the most common reasons for this can be if either the client or the host 
are behind a strict firewall. School networks tend to have this.

Luckily, you have potential ways to get around this:
- The person on the strict network can use a VPN to circumvent the firewall,
  but this will likely add input delay.
- The host can
  [port-forward for Parsec](https://support.parsec.app/hc/en-us/articles/360003146251-Starting-Port-On-The-Hosting-Computer)
  but I personally haven't had any success with this. It may also be difficult
  to convince some network administrators to forward ports.
- The host and client can use
  [Moonlight](https://moonlight-stream.org/) instead, but this requires the
  host to have an NVIDIA GPU and limits the amount of connections to just one.
  Maybe you could use Moonlight for clients on strict networks and Parsec for
  everyone else (both at the same time)...

## Client's controller doesn't work.
A client's controllers should be fine to use so as long as they themselves work
correctly on the client's own computer.

1. Double-check [Controller Compatibility](./controller-compatibility.md) to 
   see if there are any additional steps that they have to take to get their controller working.
2. If the controller works fine on the client's computer, they can try
   replugging and reconnecting their controller.
   - If the host hears that a USB device has been plugged in, chances are that 
     it'll work.