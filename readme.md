## acric <img src="assets/acric.png" alt="acric icon" width="48" height="48" align="left">
![Windows Only](https://img.shields.io/badge/platform-Windows-blue?logo=windows)
[![Downloads](https://img.shields.io/github/downloads/z1xus/acric/total)](https://github.com/z1xus/acric/releases)
[![Issues](https://img.shields.io/github/issues/z1xus/acric)](https://github.com/z1xus/acric/issues)
[![Pull Requests](https://img.shields.io/github/issues-pr/z1xus/acric)](https://github.com/z1xus/acric/pulls)

Your most realistic, fully-external autoclicker. Silent, simple and convenient.

### Features
- Realistic CPS calculation
- Hotkey support
- Fatigue simulation
- Tray icon

### Use it on your own risk.
In games, autoclicking as well as any other form of automated input is generally considered cheating in and is often punished by the game developers. I, as an author do not endorse or support cheating in games.

### Usage
1. Download the latest release from the [Releases](https://github.com/z1xus/acric/releases) page.
2. Run the executable.
3. The application will appear in the system tray.

### Modes
- Toggle Mode: Click once to start, click again to stop.
- Hold Mode: Click and hold to activate, release to stop.

You can switch between modes using the tray icon menu.

### Hotkeys
- `F6`: Toggle the app on/off
- `Back mouse button (MB5)`: Activate clicking (based on selected mode)

Note: Hotkeys cannot be changed at the moment.

### Building
1. Clone the repository
```bash
git clone https://github.com/Z1xus/acric
```
2. Build a release binary
```bash
cargo build --release
```
3. The binary will be located in the `target/release` directory

### License
This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

### Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
