# hyprspin

A lightweight, automated screen rotation daemon for **Hyprland**, written in Rust. It monitors your device's accelerometer via `iio-sensor-proxy` and automatically adjusts monitor transformation, touchscreens, tablets, and even mice to match the physical orientation.




## Features

- **Seamless Rotation**: Automatically detects `normal`, `left-up`, `right-up`, and `bottom-up` (inverted) orientations.
- **Input Sync**: Synchronizes the coordinate transformation matrix for touchscreens, tablets, and mice so your inputs always match the screen.
- **Lua Rule Engine**: Trigger specific Hyprland dispatchers or external scripts using flexible Lua scripting when an orientation is reached.
- **DPI & Scale Preservation**: Dynamically fetches current monitor states to apply rotation without breaking your resolution or scaling settings.
- **Extensible**: Easily handle complex logic (e.g., conditional execution based on monitor name or active workspace).




## Installation



### Via Cargo

You can install `hyprspin` directly from GitHub:

```bash
cargo install --git [https://github.com/naregderlevonean/hyprspin](https://github.com/naregderlevonean/hyprspin)
````



### From Source


```bash
git clone [https://github.com/naregderlevonean/hyprspin.git](https://github.com/naregderlevonean/hyprspin.git)
cd hyprspin
cargo build --release
cargo install --path .
```



### Arch GNU/Linux (AUR)


```bash
yay -S hyprspin-git
```




## Requirements

1. **iio-sensor-proxy**: Ensure the sensor daemon is installed and running:

```bash
sudo systemctl enable --now iio-sensor-proxy.service
```
    
1. **Lua**: Ensure `lua` (5.4 recommended) is installed on your system.
2. **Input Group**: Your user must have permission to manage input devices:

```bash
sudo usermod -aG input $USER
```




## Configuration

**Path:** `~/.config/hypr/hyprspin.lua`

Hyprspin now uses Lua for configuration, allowing for dynamic logic. The daemon looks for an `on_spin` function that receives a context table.



### ### The Context Object (`ctx`)

| **Field**         | **Type** | **Description**                              |
| ----------------- | -------- | -------------------------------------------- |
| `ctx.orientation` | `string` | `normal`, `left-up`, `right-up`, `bottom-up` |
| `ctx.monitor`     | `string` | The name of the monitor being rotated.       |



### Example

```lua
local PORTRAIT_MODES = { ["left-up"] = true, ["right-up"] = true }

function on_spin(ctx)
    if not ctx.monitor:find("eDP") then 
        return {} 
    end

    if ctx.orientation == "normal" then
        return {
            { action = "keyword", args = "general:layout dwindle" }
        }
    end

    if PORTRAIT_MODES[ctx.orientation] then
        return {
            { action = "keyword", args = "general:layout master" }
        }
    end

    if ctx.orientation == "bottom-up" then
        return {
            { action = "exec", args = "notify-send 'Orientation' 'Inverted (Tent Mode)'" }
        }
    end

    return {}
end
```




## Usage

Run the daemon in your `hyprland.conf`:

```hyprlang
exec-once = hyprspin
```




## License

This project is licensed under the **GPL-3.0 License**.

See the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.
