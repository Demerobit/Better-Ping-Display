# Better Ping Display (`better_ping_display`)

A native [Horizon](https://playhorizon.dev) mod that replaces ping bar icons with colored numeric millisecond values in both the **multiplayer server list** and the **tab list** (player list).

---

## Features

- **Multiplayer Server List**: Replaces the 5-bar ping sprite with the exact millisecond ping number (e.g. `28ms`, `154ms`), cleanly positioned without overlapping player counts.
- **In-Game Tab List**: Replaces the ping-bar sprites with colored numeric ping text for every player on the server.
- **Configurable Settings**: In-game toggle under **Options... → Mods → Better Ping Display** to switch server list ping numbers on or off (switching off restores standard ping bar sprites).

---

### Latency Color Thresholds

| Latency Range | Display Color | Color Code |
| :--- | :--- | :--- |
| **≤ 130 ms** | 🟢 Green | `#55FF55` (`0xFF55FF55`) |
| **131 – 220 ms** | 🟡 Yellow | `#FFFF55` (`0xFFFFFF55`) |
| **221+ ms** | 🔴 Red | `#FF5555` (`0xFFFF5555`) |
| **Unknown / Pinging** | ⬜ Gray (`???ms`) | `#AAAAAA` (`0xFFAAAAAA`) |

---

## Repository Layout

```text
Better-Ping-Display/
├── better_ping_display-0.1.0.hnmod          # Distribution package
├── better_ping_display-0.1.0.hnmod.minisig  # Minisign signature
├── .gitignore
├── LICENSE                                  # MIT License
├── README.md                                # Mod documentation
├── compatibility.json                       # Horizon revision & schema pinning
├── package.py                               # Deterministic .hnmod packaging script
└── mixins/
    └── better_ping_display/
        ├── horizon-mixin.toml               # Mixin manifest (injections & module config)
        ├── runtime.rs                       # Shared runtime state & ping formatting
        ├── sprite_scaled.rs                 # Overwrite suppressing ping bars & drawing text
        ├── server_head.rs                   # Pre-computing per-row ping values
        ├── server_return.rs                 # Mapping row coordinates to ping ms
        ├── tab_head.rs                      # Stashing player latencies in display order
        ├── tab_return.rs                    # Replacing tab-list ping sprites with text
        ├── mods.rs                          # Options > Mods list registration
        ├── settings.rs                      # Mod settings toggle button definition
        └── settings_click.rs                # Mod settings click handler
```

---

## Compatibility

| Component | Target Version / Hash |
| :--- | :--- |
| **Horizon Revision** | `66f3981f522b24c250b21523a70080965b940424` |
| **horizon-mixins Revision** | `5a64af938ec86b00d53d16a408aabc0038fb3e57` |
| **Game Version** | `26.3` |
| **Mixin Schema** | `6` |

---

## Installation & Verification

1. Download [`better_ping_display-0.1.0.hnmod`](better_ping_display-0.1.0.hnmod) and its signature.
2. Install via the Horizon Launcher's **Mods** menu or place into your Horizon mods folder.
3. Verify the package with Minisign:
   ```bash
   minisign -V -P RWSFMcL2Aq41gyFqJXfhKLk4v4oB8YMy+FTg3CakQUMEu+ZeuQcwkSgv -m better_ping_display-0.1.0.hnmod
   ```

---

## Building & Repackaging

### Repackage `.hnmod` archive
```bash
python package.py
```

### Sign with Minisign
```bash
minisign -S -s horizon-mods.key -m better_ping_display-0.1.0.hnmod -W
```

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
