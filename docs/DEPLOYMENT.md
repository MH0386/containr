# Deployment Guide

This guide covers building, packaging, and distributing Doctainr for various platforms.

## Table of Contents

- [Build for Distribution](#build-for-distribution)
- [Platform-Specific Builds](#platform-specific-builds)
- [Creating Releases](#creating-releases)
- [Distribution Channels](#distribution-channels)
- [Configuration](#configuration)
- [Updates and Versioning](#updates-and-versioning)

## Build for Distribution

### Prerequisites

Ensure you have:

- Rust toolchain (stable channel)
- Dioxus CLI (`cargo install dioxus-cli`)
- Platform-specific dependencies installed

### Release Build

```bash
# Optimized release build
dx bundle --release
```

The release binary will be located at:

- Linux: `target/dx/doctainr/release/bundle/`
- macOS: `target/dx/doctainr/release/bundle/macos/`
- Windows: `target/dx/doctainr/release/bundle/windows/`

### Build Configuration

Edit `Dioxus.toml` for build settings:

```toml
[application]
name = "doctainr"

[web.app]
title = "doctainr"

[bundle]
identifier = "com.mh0386"
publisher = "Doctainr"
icon = ["assets/icon.svg"]
```

### Optimization

For smaller binary sizes:

```bash
# Strip symbols from bundled binary
strip target/dx/doctainr/release/bundle/doctainr

# Use cargo-bloat to analyze size
cargo install cargo-bloat
cargo bloat --release
```

## Platform-Specific Builds

### Linux

#### Building on Linux

```bash
# Install dependencies
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.1-dev

# Build
dx bundle --release
```

#### Package Formats

**AppImage** (Recommended):

```bash
# Install tools
cargo install cargo-appimage

# Create AppImage
cargo appimage
```

**DEB Package**:

```bash
# Install cargo-deb
cargo install cargo-deb

# Create .deb
cargo deb

# Output: target/debian/doctainr_0.1.0_amd64.deb
```

**RPM Package**:

```bash
# Install cargo-rpm
cargo install cargo-rpm

# Initialize RPM spec
cargo rpm init

# Build RPM
cargo rpm build

# Output: target/release/rpmbuild/RPMS/x86_64/
```

**Flatpak**:

```bash
# Create flatpak manifest
# See: https://docs.flatpak.org/en/latest/

flatpak-builder build-dir com.mh0386.doctainr.yml
flatpak build-export repo build-dir
```

#### Desktop Entry

Create `/usr/share/applications/doctainr.desktop`:

```desktop
[Desktop Entry]
Name=Doctainr
Comment=Docker Desktop Management
Exec=/usr/bin/doctainr
Icon=doctainr
Terminal=false
Type=Application
Categories=Development;Utility;
```

### macOS

#### Building on macOS

```bash
# Build
dx bundle --release

# Output: target/dx/doctainr/release/bundle/macos/Doctainr.app
```

#### Code Signing

```bash
# Sign the application
codesign --force --deep --sign "Developer ID Application: Your Name" \
  target/dx/doctainr/release/bundle/macos/Doctainr.app

# Verify signature
codesign --verify --deep --strict \
  target/dx/doctainr/release/bundle/macos/Doctainr.app

# Display signature
codesign --display --verbose=4 \
  target/dx/doctainr/release/bundle/macos/Doctainr.app
```

#### Notarization

```bash
# Create DMG
create-dmg \
  --volname "Doctainr Installer" \
  --window-size 500 300 \
  --icon-size 100 \
  --app-drop-link 380 150 \
  Doctainr.dmg \
  target/dx/doctainr/release/bundle/macos/

# Notarize
xcrun notarytool submit Doctainr.dmg \
  --apple-id your-email@example.com \
  --password @keychain:notarization \
  --team-id YOUR_TEAM_ID \
  --wait

# Staple the notarization
xcrun stapler staple Doctainr.dmg
```

#### Distribution

- **Direct Download**: Provide .dmg file
- **Homebrew**: Create a Homebrew cask
- **Mac App Store**: Requires Apple Developer Program membership

### Windows

#### Building on Windows

```bash
# Build
dx bundle --release

# Output: target/dx/doctainr/release/bundle/windows/
```

#### Creating Installer

**Using WiX Toolset**:

```bash
# Install WiX
# Download from: https://wixtoolset.org/

# Create installer
candle installer.wxs
light -out Doctainr.msi installer.wixobj
```

**Using NSIS**:

```bash
# Install NSIS
# Download from: https://nsis.sourceforge.io/

# Compile installer
makensis installer.nsi
```

Example `installer.nsi`:

```nsis
!define APP_NAME "Doctainr"
!define APP_VERSION "0.1.0"
!define APP_PUBLISHER "Doctainr"
!define APP_EXE "doctainr.exe"

OutFile "Doctainr-Setup-${APP_VERSION}.exe"
InstallDir "$PROGRAMFILES64\${APP_NAME}"

Page directory
Page instfiles

Section "Install"
  SetOutPath $INSTDIR
  File "target\release\doctainr.exe"
  File "assets\icon.ico"

  CreateDirectory "$SMPROGRAMS\${APP_NAME}"
  CreateShortcut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\${APP_EXE}"
  CreateShortcut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${APP_EXE}"

  WriteUninstaller "$INSTDIR\Uninstall.exe"
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR\doctainr.exe"
  Delete "$INSTDIR\icon.ico"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"

  Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
  RMDir "$SMPROGRAMS\${APP_NAME}"
  Delete "$DESKTOP\${APP_NAME}.lnk"
SectionEnd
```

#### Code Signing

```powershell
# Sign the executable
signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com doctainr.exe

# Verify signature
signtool verify /pa doctainr.exe
```

## Creating Releases

### Version Management

Update version in `Cargo.toml`:

```toml
[package]
name = "doctainr"
version = "0.2.0"  # Bump version
```

Update `CHANGELOG.md` with release notes.

### Git Tags

```bash
# Create annotated tag
git tag -a v0.2.0 -m "Release version 0.2.0"

# Push tag
git push origin v0.2.0
```

### GitHub Releases

#### Automated with GitHub Actions

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - "v*"

jobs:
  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev
      - name: Build
        run: |
          cargo install dioxus-cli
          dx bundle --release
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: doctainr-linux
          path: target/dx/doctainr/release/bundle/

  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: |
          cargo install dioxus-cli
          dx bundle --release
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: doctainr-macos
          path: target/dx/doctainr/release/bundle/macos/

  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build
        run: |
          cargo install dioxus-cli
          dx bundle --release
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: doctainr-windows
          path: target/dx/doctainr/release/bundle/windows/

  create-release:
    needs: [build-linux, build-macos, build-windows]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v4
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            doctainr-linux/**/*
            doctainr-macos/**/*
            doctainr-windows/**/*
```

#### Manual Release

1. Build for all platforms
2. Create release on GitHub
3. Upload binaries
4. Write release notes

### Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Run tests: `cargo test`
- [ ] Build release: `dx bundle --release`
- [ ] Test release binary
- [ ] Create git tag
- [ ] Push tag
- [ ] Create GitHub release
- [ ] Upload binaries
- [ ] Announce release

## Distribution Channels

### GitHub Releases

Primary distribution method:

- Tag releases with semantic versioning
- Upload platform-specific binaries
- Include checksums for verification

### Package Managers

**Homebrew (macOS/Linux)**:

Create a Homebrew formula:

```ruby
# Formula/doctainr.rb
class Doctainr < Formula
  desc "Docker desktop management application"
  homepage "https://github.com/MH0386/doctainr"
  url "https://github.com/MH0386/doctainr/archive/v0.1.0.tar.gz"
  sha256 "..." # Calculate SHA256
  license "MIT"

  depends_on "rust" => :build
  depends_on "dioxus-cli" => :build

  def install
    system "dx", "bundle", "--release"
    bin.install "target/release/doctainr"
  end

  test do
    system "#{bin}/doctainr", "--version"
  end
end
```

**AUR (Arch Linux)**:

Create a PKGBUILD:

```bash
# PKGBUILD
pkgname=doctainr
pkgver=0.1.0
pkgrel=1
pkgdesc="Docker desktop management application"
arch=('x86_64')
url="https://github.com/MH0386/doctainr"
license=('MIT')
depends=('gtk3' 'webkit2gtk')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('...')

build() {
    cd "$pkgname-$pkgver"
    dx bundle --release
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/dx/$pkgname/release/bundle/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
```

**Cargo (Rust ecosystem)**:

```bash
# Publish to crates.io
cargo publish
```

### Direct Download

Provide direct download links:

- Linux: `.tar.gz`, `.deb`, `.rpm`, `.AppImage`
- macOS: `.dmg`
- Windows: `.msi`, `.exe` installer

## Configuration

### Application Settings

Configuration file location:

- Linux: `~/.config/doctainr/config.toml`
- macOS: `~/Library/Application Support/doctainr/config.toml`
- Windows: `%APPDATA%\doctainr\config.toml`

Example `config.toml`:

```toml
[docker]
host = "unix:///var/run/docker.sock"

[ui]
theme = "light"
auto_refresh = false
refresh_interval = 30

[window]
width = 1200
height = 800
```

### Environment Variables

Supported environment variables:

```bash
# Docker connection
DOCKER_HOST=tcp://localhost:2375

# Log level
RUST_LOG=info

# Configuration directory
DOCTAINR_CONFIG_DIR=~/.config/doctainr
```

## Updates and Versioning

### Semantic Versioning

Doctainr follows [SemVer](https://semver.org/):

- **MAJOR**: Incompatible API changes
- **MINOR**: New features (backwards-compatible)
- **PATCH**: Bug fixes (backwards-compatible)

### Update Mechanism

Future versions will include:

- Auto-update checking
- In-app update notifications
- Update download and installation

### Backward Compatibility

- Configuration files are versioned
- Migrations handle config updates
- Old versions receive security updates

## Security Considerations

### Binary Verification

Provide checksums for verification:

```bash
# Generate checksums
sha256sum doctainr-linux-x86_64.tar.gz > checksums.txt
sha256sum doctainr-macos.dmg >> checksums.txt
sha256sum doctainr-windows.msi >> checksums.txt

# Verify
sha256sum -c checksums.txt
```

### Code Signing

Always sign binaries for:

- macOS: Required for notarization
- Windows: Increases user trust
- Linux: AppImage signing

### Docker Socket Security

Document Docker socket permissions:

- Recommend docker group membership
- Warn about security implications
- Provide alternatives (TCP with TLS)

## Monitoring and Analytics

### Crash Reporting

Consider integrating:

- Sentry for error tracking
- Custom telemetry (opt-in only)

### Usage Metrics

If collecting metrics:

- Always opt-in
- Transparent about data collection
- Anonymize user data
- Provide opt-out mechanism

## Support

### Documentation

Ensure documentation is:

- Updated with each release
- Available offline
- Versioned

### User Support

Provide support through:

- GitHub Issues
- Discussion forums
- Email support
- FAQ documentation

## License and Legal

### License File

Include `LICENSE` file in all distributions.

### Third-Party Licenses

Include attribution for dependencies:

```bash
# Generate license information
cargo install cargo-license
cargo license --authors --do-not-bundle --avoid-build-deps > THIRD_PARTY_LICENSES.txt
```

### Privacy Policy

If collecting any data, provide a privacy policy.

---

For more information, see:

- [Development Guide](DEVELOPMENT.md)
- [Architecture Documentation](../ARCHITECTURE.md)
- [Contributing Guide](../CONTRIBUTING.md)
