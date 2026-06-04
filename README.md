# follow-focus

Warp the pointer to the centre of the focused window on demand.

This is useful in [Sway](https://swaywm.org/) setups where keyboard-driven focus
changes can leave the mouse pointer behind on another window or output.

`follow-focus` listens for Sway window focus events and remembers the focused
window's rectangle. When it receives a custom Sway tick event, it moves the
pointer to the centre of the last focused window. Because the pointer only moves
in response to the tick — not on every focus change — it stays out of the way
during normal mouse use and only catches up when *you* ask it to.

## Requirements

- [Sway](https://swaywm.org/)

## Install

With Cargo, directly from GitHub:

```sh
cargo install --git https://github.com/dstoc/follow-focus
```

This installs the `follow-focus` binary into `~/.cargo/bin`.

## Quick start

Add the following to your Sway config (`~/.config/sway/config`):

```
# Run the daemon
exec_always follow-focus

# Fire the tick that tells follow-focus to warp the pointer
set $followfocus ; exec swaymsg -q -t send_tick follow_focus

# Append $followfocus to keybindings that change focus
bindsym $mod+$left  focus left  $followfocus
bindsym $mod+$down  focus down  $followfocus
bindsym $mod+$up    focus up    $followfocus
bindsym $mod+$right focus right $followfocus
```

Reload Sway (`$mod+Shift+c`). Now moving focus with the keyboard brings the
pointer along to the centre of the newly focused window.

## How it works

1. On startup, `follow-focus` opens two Sway IPC connections — one to issue
   commands, one to stream events — and subscribes to window and tick events.
2. On every window **focus** event it records the focused window's rectangle.
   No pointer movement happens here.
3. On a **tick** event with the payload `follow_focus`, it warps the pointer to
   the centre of the last recorded rectangle via
   `seat - cursor set <x> <y>`.

The keybindings above send that tick after changing focus, so the pointer
follows focus only when you drive it from the keyboard.
