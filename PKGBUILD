# Maintainer: JustASpeedrunner <justasped@stepbro.wtf>
pkgname=screenctrl
pkgver=1.0.0
pkgrel=1
pkgdesc="A GUI tool to manage your monitor's brightness, using brightnessctl, and color temprature, using xsct, in Xorg."
arch=('x86_64')
url="https://justaspeedrunner.github.io/screenctrl.html"
license=('MIT')
depends=('brightnessctl' 'xorg-xrandr' 'xsct' 'libx11' 'libxrandr')
makedepends=('git' 'cargo-nightly')
optdepends=('ddcci-driver-linux-dkms: multi-monitor support')
source=("$pkgname::git://github.com/JustASpeedrunner/screenctrl.git")
sha512sums=('SKIP')

build() {
  cd "$pkgname"
  cargo build --release --locked
}

package() {
  cd "$pkgname"
  local OUT_DIR=$(<out_dir)

  install -Dm755 "target/release/screenctrl" "$pkgdir/usr/bin/screenctrl"

  install -Dm644 "README.md" "$pkgdir/usr/share/doc/${pkgname}/README.md"
  install -Dm644 "LICENSE-MIT" "$pkgdir/usr/share/licenses/${pkgname}/LICENSE-MIT"
}
