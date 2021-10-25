# Issues
This documentation is intended to compile all known and potential issues into a 
single place. If an issue already has an open GitHub issue associated with it, 
the main one will be hyperlinked. Please consult this before
[making a report](https://github.com/kenesu-h/sys-hidplus-client-rs/issues)
to see if your issue has already been found and resolved. If one can't be fixed,
these tables will at least attempt to list possible workarounds.

# Known Issues
This section contains a table of issues that I am already aware of.

<table style="width: 100%">
  <tr>
    <th style="width: 50%">Issue</th>
    <th style="width: 50%">Notes</th>
  </tr>
  <tr>
    <td>
      <a href="https://github.com/kenesu-h/sys-hidplus-client-rs/issues/">
        Input delay in demanding games.
      </a>
    </td>
    <td>
      This issue hasn't been tested very much, but my guess is that there is no 
      fix for this issue given that it happened to the original sysmodule. The 
      best you could do is probably deal with it or not play the game causing 
      the delay. 
    </td>
  </tr>
  <tr>
    <td>
      <a href="https://github.com/kenesu-h/sys-hidplus-client-rs/issues/1">
        Commands not recognized when using arrow keys in command line.
      </a>
    </td>
    <td>
      This isn't a big issue if you don't use the command line. I don't know 
      how to fix this, but avoid using arrow keys in the command line. If you
      accidentally used one, try to move your cursor all the way to the last
      line.
    </td>
  </tr>
</table>

# Potential Issues
This section contains a table of issues that may theoretically happen, but are
not confirmed to exist. If you feel like you've encountered an issue listed
here, feel free to make a report.

<table style="width: 100%">
  <tr>
    <th style="width: 50%">Issue</th>
    <th style="width: 50%">Notes</th>
  </tr>
  <tr>
    <td>More input delay when using GUI.</td>
    <td>
      The command line interface is available as an alternative and will likely
      have minimal input delay by comparison.
    </td>
  </tr>
  <tr>
    <td>More input delay with more players.</td>
    <td>
      I have only tested with four concurrent players at most, but there was no
      noticeable input delay. However, this may not be the same case for more 
      than four players, as well as hosts with weaker computers.
  </tr>
</table>