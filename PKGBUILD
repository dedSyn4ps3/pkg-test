# Maintainer: Ed Rutherford <erutherford@nullsecurity.tech>
pkgname=pkg-test
pkgver=1.0.2
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
)
provides=("pkg-test")
conflicts=("pkg-test")
source=("$pkgname::git+https://github.com/dedSyn4ps3/pkg-test.git")
sha256sums=("SKIP")

build() {
    cd "$pkgname"

    echo "[+] Installing Rust Nightly…"
    rustup toolchain install nightly
    rustup default nightly

    echo "[+] Gathering UI dependencies..."
    npm install

    echo "[*] STARTING BUILD [*]"
    npm run tauri build
}

package() {
    cd "${pkgname}/src-tauri/target/release/bundle/deb/${pkgname}_${pkgver}_amd64/data"

    for size in 128x128 256x256@2 512x512; do
        install -Dm644 "usr/share/icons/hicolor/${size}/apps/${pkgname}.png" "${pkgdir}/usr/share/icons/hicolor/${size}/apps/${pkgname}.png"
    done

    install -Dm644 "usr/share/desktop/${pkgname}.desktop" "${pkgdir}/usr/share/applications/${pkgname}.desktop"

    install -Dm755 "usr/bin/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"

    install -Dm755 "usr/share/scripts/*" "${pkgdir}/usr/share/${pkgname}/scripts"

    echo
    echo "[*] PKGBUILD COMPLETE [*]"
}
