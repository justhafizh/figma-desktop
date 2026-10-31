# Figma Desktop

A lightweight Figma desktop app for Linux, built with Tauri and WebKitGTK.

> **Disclaimer:** This is an **unofficial** project. It is not affiliated
> with, endorsed by, or sponsored by Figma, Inc. "Figma" is a trademark
> of Figma, Inc.

## Tech Stack

| Tech | Fungsi |
|------|--------|
| **Tauri** | Framework untuk membungkus web jadi aplikasi desktop (ringan, pakai Rust) |
| **Rust** | Backend Tauri: membuat window, mengatur izin |
| **WebKitGTK** | Web engine untuk menampilkan figma.com di dalam window |

## Prerequisites

- Ubuntu / Linux
- Node.js & npm
- Rust (via rustup)
- Dependency sistem Tauri:

```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

## Setup

```bash
git clone https://github.com/justhafizh/figma-desktop.git
cd figma-desktop
npm install
```

## Run (development)

```bash
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

Hasil ada di `src-tauri/target/release/bundle/`:

- `deb/` → file `.deb`
- `appimage/` → file `.AppImage`

Install `.deb`:

```bash
sudo apt install ./nama-file.deb
```

Jalankan AppImage:

```bash
chmod +x nama-file.AppImage
./nama-file.AppImage
```

## License

MIT
