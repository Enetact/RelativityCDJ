# RelativityCDJ

RelativityCDJ is a high-performance, cross-platform DJ emulator designed to replicate the functionality of professional CDJ systems. Built with Rust and `eframe/egui`, it offers a modern GUI-based digital DJ interface focused on performance, modularity, and extensibility.

---

## 🎯 Project Goals

- Deliver a native, low-latency digital DJ interface with waveform visualization and audio manipulation.
- Provide modular architecture for customizable audio decks, FX, and UI components.
- Offer future support for real-time file system monitoring and music library management.

---

## 🏗 Architecture Overview

```
RelativityCDJ/
│
├── src/
│   ├── app.rs             # Core application state and logic
│   ├── main.rs            # Entry point - bootstraps the UI using eframe
│   ├── audio/
│   │   ├── beatgrid.rs    # BPM grid alignment for tracks
│   │   ├── deck.rs        # Deck management (load/play tracks)
│   │   ├── mix.rs         # Mixer logic
│   │   ├── fx.rs          # Audio effects engine
│   │   └── waveform.rs    # Waveform rendering
│   └── ui/
│       ├── library.rs     # Library UI panel
│       ├── mixer.rs       # Mixer UI controls
│       ├── cdj.rs         # Virtual CDJ interface
│       └── footer.rs      # Status/footer section
│
├── Cargo.toml             # Rust dependencies
├── .env                   # Environment configuration
```

---

## 🚀 Features (Current & Roadmap)

| Feature                     | Status       |
|----------------------------|--------------|
| Dual audio deck control    | ✅ Implemented |
| Waveform visualization     | ✅ Basic support |
| FX unit system             | ✅ Modular FX |
| Scrollable track library   | ✅ Static only |
| Local music folder scan    | 🔜 Planned |
| Audio file metadata parser | 🔜 Planned |
| Track sync + quantize      | 🔜 Planned |
| Export cue points/grid     | 🔜 Planned |

---

## 🧰 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+ recommended)
- Git

---

## 🛠️ Build & Run

```bash
# Clone the repository
git clone https://github.com/Enetact/RelativityCDJ.git
cd RelativityCDJ

# Build and run
cargo run --release
```

> **Note**: Windows, Linux, and macOS supported natively via `eframe`.

---

## 📁 Music Folder Support (Planned)

In a future release, RelativityCDJ will scan your OS-specific music folder automatically:

- **Windows:** `%USERPROFILE%\Music`
- **macOS:** `~/Music`
- **Linux:** `~/Music`

A file system watcher and metadata parser (ID3, etc.) will populate the track library dynamically.

---

## 👥 Contributing

We welcome contributions! Please follow our coding standards:

- Rustfmt + Clippy clean code
- Descriptive PRs and commit messages
- All new features must include unit or integration tests

To get started:

```bash
git checkout -b feature/<your-feature>
# make your changes
git commit -m "Add <your feature>"
git push origin feature/<your-feature>
```

---

## 📄 License

[MIT License](LICENSE)

---

## 💬 Contact

Maintained by [Enetact Labs](https://github.com/Enetact). For enterprise collaboration or integration, please open an issue or contact directly.