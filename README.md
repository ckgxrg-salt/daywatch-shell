# dwsh - Daywatch Shell

A desktop shell with GTK4 for my laptop

Since I'm using Hyprland now, these require Hyprland IPC to work.

## Log out

Usage:

```shell
$ dwsh-logout
```

This will create an overlay panel with 5 possible actions:
(can also be selected by pressing character shown in brackets)
- Power off(s)
- Reboot(r)
- Log out(e)
- Suspend(u)
- Lock screen(l)

Selecting a button for the first time will highlight it, and again will confirm and execute the action.
Press Esc or click anywhere else to deselect, and again to quit the panel.
