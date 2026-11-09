# Better Ping Display (`better_ping_display`)

A native [Horizon](https://playhorizon.dev) mod that replaces ping bar icons with colored numeric millisecond values in both the **multiplayer server list** and the **tab list** (player list).

---

## Features

- **Multiplayer Server List**: Replaces the 5-bar ping sprite with the exact millisecond ping number (e.g. `28ms`, `154ms`), cleanly positioned without overlapping player counts.
- **In-Game Tab List**: Replaces ping bar sprites with colored numeric ping text for every player on the server, sizing and right-aligning the text within column boundaries so adjacent columns never overlap.
- **Configurable Settings**: In-game toggle under **Options... → Mods → Better Ping Display** to switch server list ping numbers on or off (switching off restores standard ping bar sprites).

---

### Latency Color Thresholds

| Latency Range | Display Color | Color Code |
| :--- | :--- | :--- |
| **≤ 130 ms** | 🟢 Green | `#55FF55` (`0xFF55FF55`) |
| **131 – 220 ms** | 🟡 Yellow | `#FFFF55` (`0xFFFFFF55`) |
| **221+ ms** | 🔴 Red | `#FF5555` (`0xFFFF5555`) |
| **Unknown / Pinging** | ⬜ Gray (`?ms`) | `#AAAAAA` (`0xFFAAAAAA`) |

---

## Repository Layout

```text
Better-Ping-Display/
├── .gitignore
├── LICENSE                           # MIT License
├── README.md                         # Mod documentation
├── compatibility.json                # Horizon revision & schema pinning
├── package.py                        # Deterministic .hnmod packaging script
└── mixins/
    └── better_ping_display/
        ├── horizon-mixin.toml        # Mixin manifest (injections & module config)
        ├── runtime.rs                # Shared runtime state & ping formatting
        ├── sprite_scaled.rs          # Suppressing server ping bars & drawing colored numeric text
        ├── text_styled.rs            # Shifting server player count left when ping text expands
        ├── server_head.rs            # Stashing per-row ping values before draw
        ├── server_return.rs          # Mapping row coordinates to ping ms
        ├── tab_head.rs               # Measuring player latency widths & stashing layout info
        ├── tab_return.rs             # Reserving column widths & right-aligning numeric pings
        ├── mods.rs                   # Options > Mods list registration
        ├── settings.rs               # Mod settings toggle button definition
        └── settings_click.rs         # Mod settings click handler
```

---

## Compatibility

| Component | Target Version / Hash |
| :--- | :--- |
| **Horizon Revision** | `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca` |
| **horizon-mixins Revision** | `5a64af938ec86b00d53d16a408aabc0038fb3e57` |
| **Game Version** | `26.3` |
| **Mixin Schema** | `6` |

---

## Installation

1. Download [`better_ping_display-0.1.0.hnmod`](https://github.com/Demerobit/Better-Ping-Display/releases/download/v0.1.0/better_ping_display-0.1.0.hnmod) from the [Releases](https://github.com/Demerobit/Better-Ping-Display/releases) page.
2. Place the file into your Horizon native mods folder (`%APPDATA%\horizon\horizon-native-mods\`) or install it directly in the Horizon Launcher.

---

## Building & Packaging

### Package `.hnmod` archive
```bash
python package.py
```

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
