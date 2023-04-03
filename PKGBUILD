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
    "nodejs-lts-gallium"
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
#source=("$pkgname::file:///home/eddiesneed/Documents/pkg-test")
sha256sums=("SKIP")

build() {
    PURPLE=$(tput setaf 201)
    WHITE=$(tput setaf 255)
    END="\e[0m"
    cd "$pkgname"

    echo
    echo -e "${PURPLE}|============================|${END}"
    echo -e "${WHITE}   Installing Rust Nightly     ${END}"
    echo -e "${PURPLE}|============================|${END}"

    rustup toolchain install nightly
    rustup default nightly

    echo
    echo -e "${PURPLE}|=============================|${END}"
    echo -e "${WHITE}   Gathering UI Dependencies     ${END}"
    echo -e "${PURPLE}|=============================|${END}"

    npm install

    echo
    echo -e "${PURPLE}|=============================|${END}"
    echo -e "${WHITE}     Compiling Application     ${END}"
    echo -e "${PURPLE}|=============================|${END}"

    npm run tauri build
}

package() {
    cd "${pkgname}/src-tauri/target/release/bundle/deb/${pkgname}_${pkgver}_amd64/data"

    for size in 128x128 256x256@2 512x512; do
        install -Dm644 "usr/share/icons/hicolor/${size}/apps/${pkgname}.png" "${pkgdir}/usr/share/icons/hicolor/${size}/apps/${pkgname}.png"
    done

    install -Dm644 "usr/share/desktop/${pkgname}.desktop" "${pkgdir}/usr/share/applications/${pkgname}.desktop"

    install -Dm755 "usr/bin/${pkgname}" "${pkgdir}/usr/bin/${pkgname}"

    install -Dm755 "usr/share/scripts/update_system.sh" "${pkgdir}/usr/share/${pkgname}/scripts/update_system.sh"

    echo
    echo -e "${PURPLE}|=============================|${END}"
    echo -e "${WHITE}     Packaging Complete     ${END}"
    echo -e "${PURPLE}|=============================|${END}"
    echo
}
