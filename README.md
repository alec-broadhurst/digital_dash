# Digital Dash

**Digital Dash** is a Rust-based digital dashboard that runs on your computer and visualizes real-time telemetry data from racing simulators. The tool is designed for scalability, allowing future support for multiple racing games.

## Features

- **Real-time Telemetry**: Displays live data such as RPM, speed, lap times, gear, and more from supported games.
- **Modular Design**: The codebase is structured to easily add support for additional racing simulators in the future.

## Current Support

- **Forza**: The dashboard currently supports telemetry data from Forza Motorsport 7/8.

## Planned Features

- **Support for Additional Games**: Future updates will add support for telemetry data from other popular racing games.
- **Expanded Telemetry Metrics**: As new games are added, game-specific telemetry data (e.g., ERS for F1) will be supported.

## How It Works

- The tool reads telemetry data over UDP, processes it, and then updates the dashboard with real-time performance data from the game.
- A modular architecture ensures that different game telemetry formats can be added easily, without affecting the core functionality.

## Usage

Currently, the project is under development and supports only Forza Motorsport telemetry data. More games and features will be added in future releases.

### License

This project is licensed under the MIT License.
