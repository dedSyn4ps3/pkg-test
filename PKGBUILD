# Maintainer: Ed Rutherford <erutherford@nullsecurity.tech>
pkgname=pkg-test
pkgver=1.0.0
pkgrel=1
pkgdesc="For testing Tauri app script execution"
arch=('x86_64')
url="https://github.com/dedSyn4ps3/pkg-test"
license=('MIT')
depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'pango' 'webkit2gtk')
makedepends=(
    "npm"
    "nodejs"
    "rustup"
    "git"
    "webkit2gtk"
    "base-devel"
    "curl"
    "wget"
    "openssl"
    "appmenu-gtk-module"
    "gtk3"
    "libappindicator-gtk3"
    "librsvg"
    "libvips"
    "debtap"
)
provides=("pkg-test")
conflicts=("pkg-test")
source=("$pkgname::git+https://github.com/dedSyn4ps3/pkg-test.git")
sha256sums=("SKIP")

build() {
    cd "$pkgname-$pkgver"

    echo "[+] Installing Rust Nightly…"
    rustup toolchain install nightly
    rustup default nightly

    echo "[+] Gathering UI dependencies..."
    npm install

    echo "[*] STARTING BUILD [*]"
    npm run tauri build
}

package() {
    cd "$pkgname-$pkgver"
    cd "src-tauri/target/release/bundle/deb"
    debtap -Q "${pkgname}_${pkgver}_amd64.deb"

    cp "${pkgname}-${pkgver}-${pkgrel}-.pkg.tar.zst" "$pkgname-$pkgver"
}
