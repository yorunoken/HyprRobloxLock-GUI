# HyprRobloxLock-GUI

A graphical user interface (GUI) version of the HyprRobloxLock tool for managing cursor locking in Waydroid while playing Roblox.

**Disclaimer: Only works on Hyprland.**

## Prerequisites

- hyprland
- dotool

## Usage

1. **Installation**
   - Download the binary from the [releases tab](https://github.com/yorunoken/HyprRobloxLock-GUI/releases) and place it in `/usr/bin`.
   - Or use the AUR if you're on Arch Linux

2. **Running the Application**
   - Launch the application by running `hypr-roblox-lock-gui` from the terminal.

3. **Configuration in the GUI**
   - **Edge Offset**: This value defines how close the cursor can get to the edge of the screen before it gets repositioned.

4. **Controls**
   - **Toggle Lock**: Use the button labeled "Toggle lock ON" or "Toggle lock OFF" to start or stop the cursor locking functionality.
   - **Force Kill**: If needed, use the "Force kill" button to abruptly terminate the application.

## Important

If you encounter issues with the application or need to terminate it, use the following Hyprland configuration to bind the termination command:

```sh
bind = CTRL SHIFT, X, exec, killall hypr-roblox-lock-gui
```

## Known Issues

- There is no built-in way to terminate the application via a key input within the app itself. Please use the provided keybind or force kill option.
