# Maintainer: Max Lafrance maxlafrance97@gmail.com

pkgname() {
    grep '^name =' Cargo.toml | cut -d'"' -f2
}    

pkgver() {
    grep '^version =' Cargo.toml | cut -d'"' -f2
}    

pkgdesc {
    grep '^description =' Cargo.toml
}    

pkgrel=1

arch=('x86_64')
url="https://github.com/mdlafrance/btui"
license=('MIT' or 'GPL')
makedepends=('cargo rust dbus')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
}
