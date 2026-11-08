# Better Ping Display

A [Horizon](https://horizon.gg/) native mod that replaces the ping-bar icons with coloured millisecond numbers in both the **server list** and the **tab list** (in-game player list).

## Features

- **Server list** – shows exact ping in ms (e.g. `42ms`) to the right of the bar icon, coloured by latency.
- **Tab list** – replaces the ping-bar sprite with a coloured ms number for every player.
- **Settings toggle** – open *Mods → Better Ping Display* in-game to turn the server-list numbers on or off.

### Colour thresholds

| Colour | Range |
|--------|-------|
| 🟢 Green | 0 – 130 ms |
| 🟡 Yellow | 131 – 220 ms |
| 🔴 Red | 221 ms + |
| ⬜ Gray | Unknown / pinging |

## Compatibility

| Field | Value |
|-------|-------|
| Horizon revision | `66f3981f522b24c250b21523a70080965b940424` |
| horizon-mixins revision | `5a64af938ec86b00d53d16a408aabc0038fb3e57` |
| Game version | 26.3 |

## Installation

Download `better_ping_display-0.1.0.hnmod` from the [latest release](../../releases/latest) and install it via the Horizon launcher's mod manager.

Verify the file with the public key:

```
RWSFMcL2Aq41gyFqJXfhKLk4v4oB8YMy+FTg3CakQUMEu+ZeuQcwkSgv
```

## License

MIT
