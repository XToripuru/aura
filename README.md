
# Aura

**Aura** is a minimalistic, terminal-based launcher for [Hyprland](https://hypr.land), designed to be fast and simple.

---

## Setup

### 1. Configure
Modify `src/config.rs` to customize the launcher’s behavior, or leave it as-is for the default experience.

### 2. Build
```bash
cargo build --release
````

### 3. Install

```bash
sudo mv target/release/aura /usr/local/bin/
```

### 4. Integrate with Hyprland

#### `keybindings.conf`

Add a keybinding to launch Aura from your terminal (change `alacritty` if you use a different terminal):

```ini
$mainMod = SUPER
...
bind = $mainMod, R, exec, alacritty --class launcher -e /usr/local/bin/aura
```

#### `windowrule.conf`

Ensure the launcher opens centered and floating:

```ini
windowrulev2 = float, class:^(launcher)$
windowrulev2 = center, class:^(launcher)$
```

---

## Contributing

Contributions are welcome! Feel free to fork the project, submit pull requests, or open issues for ideas, improvements, or bugs.

---

## License

This project is released under the [Unlicense](https://unlicense.org/), meaning **you can do anything you want** with it — no restrictions.