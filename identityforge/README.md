# IdentityForge

**Anti-Detect Browser Manager** - A desktop application built with Tauri 2.0 and Rust that launches isolated WebView windows with completely unique, spoofed digital fingerprints.

## Features

### Profile Management
- **Create Profiles**: Generate new browser identities with auto-generated fingerprints
- **Platform Selection**: Choose Windows, macOS, or Linux fingerprints (or random)
- **Edit Profiles**: Customize all fingerprint parameters manually
- **Delete Profiles**: Remove profiles and all associated browsing data

### Fingerprint Spoofing (The "Secret Sauce")
- **User Agent**: Realistic browser/OS combinations (Chrome, Firefox, Safari)
- **Screen Resolution**: Common display sizes from 1280x720 to 4K
- **Hardware Concurrency**: Spoofed CPU core count (2-32 cores)
- **Device Memory**: Spoofed RAM (2-64 GB)
- **WebGL Vendor/Renderer**: Intel, NVIDIA, AMD, Apple GPU strings
- **Canvas Fingerprint**: Noise injection to randomize canvas hash
- **Timezone**: Spoofed timezone with proper offset calculation
- **Language**: Browser language preferences

### Browser Isolation
- **Separate Data Directories**: Each profile uses `profiles/{id}/` for cookies, localStorage, IndexedDB
- **No Cookie Sharing**: Complete isolation between profiles
- **WebRTC Protection**: Disabled to prevent IP leaks

### User Interface
- **Modern Dark UI**: Built with React + TailwindCSS
- **Profile Cards**: Visual overview of all profiles with status indicators
- **Search & Filter**: Find profiles by name, filter by active/inactive
- **Launch with Custom URL**: Start browser sessions at any URL

## Tech Stack

| Component | Technology |
|-----------|------------|
| Framework | Tauri 2.0 |
| Backend | Rust |
| Frontend | React + TypeScript |
| Styling | TailwindCSS |
| Database | SQLite (rusqlite) |
| Build Tool | Vite |

## Project Structure

```
identityforge/
├── src/                      # React frontend
│   ├── components/           # UI components
│   │   ├── ProfileCard.tsx
│   │   ├── CreateProfileModal.tsx
│   │   └── EditProfileModal.tsx
│   ├── hooks/
│   │   └── useApi.ts         # Tauri command bindings
│   ├── types/
│   │   └── profile.ts        # TypeScript interfaces
│   ├── App.tsx               # Main application
│   └── main.tsx              # Entry point
├── src-tauri/                # Rust backend
│   ├── src/
│   │   ├── lib.rs            # Tauri setup
│   │   ├── main.rs           # Entry point
│   │   ├── commands.rs       # Tauri commands
│   │   ├── database.rs       # SQLite operations
│   │   ├── fingerprint.rs    # Fingerprint generation & JS injection
│   │   └── launcher.rs       # WebView window management
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── tailwind.config.js
└── vite.config.ts
```

## Prerequisites

### System Dependencies (Linux)
```bash
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libsoup-3.0-dev \
  libjavascriptcoregtk-4.1-dev
```

### Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Node.js & pnpm
```bash
# Install Node.js 18+ and pnpm
npm install -g pnpm
```

### Tauri CLI
```bash
cargo install tauri-cli --version "^2.0"
```

## Installation

1. **Clone the repository**
```bash
git clone <repository-url>
cd identityforge
```

2. **Install frontend dependencies**
```bash
pnpm install
```

3. **Build the application**
```bash
# Development mode
pnpm tauri dev

# Production build
pnpm tauri build
```

## Usage

### Creating a New Identity

1. Click **"New Identity"** button
2. Enter a profile name (e.g., "FB Ad Account 1")
3. Optionally select a platform (Windows/Mac/Linux)
4. Click **"Preview Fingerprint"** to see generated values
5. Click **"Create Identity"**

### Launching a Profile

1. Find the profile card in the grid
2. (Optional) Click **"URL"** to set a custom start URL
3. Click **"Launch"**
4. A new isolated browser window opens with the spoofed fingerprint

### Managing Profiles

- **Edit**: Click the pencil icon to modify fingerprint values
- **Regenerate**: Click the refresh icon to generate new fingerprint
- **Delete**: Click the trash icon to remove profile and data
- **Close**: Click "Close Window" on active profiles

## Fingerprint Spoofing Details

### JavaScript Injection

Each browser window receives an initialization script that overwrites:

```javascript
// Navigator properties
navigator.userAgent
navigator.platform
navigator.hardwareConcurrency
navigator.deviceMemory
navigator.language
navigator.languages

// Screen properties
screen.width
screen.height
screen.availWidth
screen.availHeight

// WebGL
WebGLRenderingContext.prototype.getParameter (UNMASKED_VENDOR/RENDERER)

// Canvas (noise injection)
HTMLCanvasElement.prototype.toDataURL
HTMLCanvasElement.prototype.toBlob

// WebRTC (disabled)
RTCPeerConnection

// Timezone
Date.prototype.getTimezoneOffset
Intl.DateTimeFormat
```

### Canvas Fingerprint Protection

The canvas fingerprint is randomized by injecting subtle pixel noise:
- Only modifies every Nth pixel
- Changes are imperceptible visually
- Results in unique canvas hash per profile

### WebRTC Leak Prevention

WebRTC is neutered by:
- Clearing ICE servers configuration
- Preventing STUN/TURN connections
- Blocking local IP discovery

## Extensibility

The database includes tables for future plugin support:
- `plugins` table for addon management
- `settings` table for configuration storage

## API Reference

### Tauri Commands

| Command | Description |
|---------|-------------|
| `get_profiles` | Get all profiles with active status |
| `get_profile` | Get single profile by ID |
| `create_profile` | Create new profile with fingerprint |
| `update_profile` | Update profile fields |
| `delete_profile` | Delete profile and data |
| `regenerate_fingerprint` | Generate new fingerprint for profile |
| `launch_profile` | Open isolated browser window |
| `close_profile_window` | Close profile's browser window |
| `navigate_profile` | Navigate window to URL |
| `preview_fingerprint` | Generate fingerprint without saving |

## Security Considerations

- Profiles are isolated but share the same application process
- WebRTC is disabled to prevent IP leaks
- Canvas fingerprinting is mitigated with noise injection
- Each profile has its own cookie jar and storage

## License

MIT License

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

---

**IdentityForge** - Manage multiple browser identities with unique fingerprints.
