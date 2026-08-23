<div align="center">

# Chumpkin

![CI](https://github.com/callenflynn/Chumpkin/actions/workflows/rust.yml/badge.svg)
[![Discord](https://img.shields.io/discord/1268592337445978193.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](URL HERE) (coming soon)
[![License: GPL](https://img.shields.io/badge/License-GPLv3-yellow.svg)](https://opensource.org/licenses/gpl-3-0)

</div>

[Chumpkin](https://github.com/callenflynn/Chumpkin) is a Minecraft server built entirely in Rust, offering a fast, efficient,
and customizable experience. It prioritizes performance and player enjoyment while adhering to the core mechanics of the game.
<div align="center">

![Chumpkin Chunk Loading](./assets/pumpkin-chunk-loading.webp)

</div>

## Goals

- **Performance**: Leveraging multi-threading for maximum speed and efficiency.
- **Compatibility**: Supports the latest Java & Bedrock Minecraft server version while adhering to Vanilla game mechanics.
- **Security**: Prioritizes security by preventing known security exploits.
- **Flexibility**: Highly configurable, with the ability to disable unnecessary features.
- **Extensibility**: Provides a foundation for plugin development.

> [!IMPORTANT]
> Chumpkin is currently under heavy development.
>
> [See what needs to be done before the 1.0.0 Release](https://github.com/callenflynn/Chumpkin/issues/449)

## Features

- [x] Configuration (toml)
- [Tracking: Protocol](https://github.com/callenflynn/Chumpkin/issues/1401)
  - [x] Server Status/Ping
  - [x] Encryption
  - [x] Packet Compression
  - [x] Java Edition
  - [x] Bedrock Edition (W.I.P)
  - ...
- [Tracking: World](https://github.com/callenflynn/Chumpkin/issues/1403)
  - [x] Player Tab-list
  - [x] Scoreboard
  - [x] World Loading
  - [x] World Time
  - [x] World Borders
  - [x] World Saving
  - [x] Lighting
  - [x] Entity Spawning
  - [x] Bossbar
  - [x] Chunk Loading (Vanilla, Linear, Pump)
  - [Chunk Generation](https://github.com/callenflynn/Chumpkin/issues/36)
  - [x] Chunk Saving (Vanilla, Linear, Pump)
  - [Redstone](https://github.com/callenflynn/Chumpkin/issues/1402)
  - [x] Liquid Physics
  - ...
- [Tracking: Player](https://github.com/callenflynn/Chumpkin/issues/1405)
  - [x] Skins
  - [x] Teleport
  - [x] Movement
  - [x] Animation
  - [x] Inventory
  - [Combat](https://github.com/callenflynn/Chumpkin/issues/1404)
  - [x] Experience
  - [x] Hunger
  - [X] Off Hand
  - [X] Advancements (W.I.P)
  - [x] Eating
  - ...
- Entities
  - [x] Non-Living (Minecart, Eggs...) (W.I.P)
  - [x] Entity Effects
  - [x] Players
  - [x] Mobs (W.I.P)
  - [x] Animals (W.I.P)
  - [Entity AI](https://github.com/callenflynn/Chumpkin/issues/1406)
  - [x] Boss (W.I.P)
  - [x] Villagers (W.I.P)
  - [X] Entity Saving
- Server
  - [Plugins](https://github.com/callenflynn/Chumpkin/issues/1407)
  - [x] Query
  - [x] RCON
  - [x] Inventories
  - [x] Particles
  - [x] Chat
  - [Commands](https://github.com/callenflynn/Chumpkin/issues/15)
  - [x] Permissions
  - [x] Translations
- Proxy
  - [x] Bungeecord
  - [x] Velocity

<!-- Check out our [Github Project](https://github.com/callenflynn/Chumpkin/projects) to see current progress. -->

## How to run

See our [Quick Start](https://github.com/callenflynn/Chumpkin#how-to-run) guide to get Chumpkin running.

### World types

Set the world generator in `configuration.toml` under `[world]`:

```toml
[world]
world_type = "chumpkin"  # "normal" | "flat" | "chumpkin"
```

| Type | Description |
|------|-------------|
| `normal` | Vanilla noise-based terrain, 384 blocks tall (default). |
| `flat` | Superflat world, configured via `world_gen_settings.dat`. |
| `chumpkin` | 2048 blocks tall, mountains reach ~1500. Same noise, taller ceiling. |

## Contributions

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md)

## Docs

Chumpkin's documentation can be found at <https://github.com/callenflynn/Chumpkin>

## Communication

Consider joining [our Discord server](https://discord.gg/wT8XjrjKkf) to stay up-to-date on events, updates, and connect with other members.

## Funding

If you want to fund me and help the project, check out the [Donation Page](https://github.com/callenflynn/Chumpkin).

## License & Attribution

* **Chumpkin Server**: Licensed under the [GNU General Public License v3.0 (GPLv3)](LICENSE).
* **Plugin API (`pumpkin-plugin-api` & `pumpkin-plugin-wit`)**: Dual-licensed under [MIT](crates/pumpkin-plugin-api/LICENSE-MIT) OR [Apache-2.0](crates/pumpkin-plugin-api/LICENSE-APACHE) for maximum flexibility when writing plugins.
* **Third-Party Assets & Data**: Bedrock mappings, protocol conversion data, and Minecraft assets are subject to their respective licenses and attribution terms. See [assets/NOTICE.md](assets/NOTICE.md) for full details.
