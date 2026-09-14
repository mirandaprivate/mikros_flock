#!/usr/bin/env bash
# Local-only FLINT build for Ubuntu 22.04 x86_64; no system package installation.
set -euo pipefail
cd -- "$(dirname -- "$0")"
root=$(pwd)
mkdir -p deps/downloads deps/ubuntu deps/prefix/include deps/prefix/lib build
cd deps/downloads
if [[ ! -f flint-3.6.0.tar.gz ]]; then
    curl -fL --retry 2 https://flintlib.org/download/flint-3.6.0.tar.gz -o flint-3.6.0.tar.gz
fi
if [[ ! -f ../ubuntu/usr/include/mpfr.h ]]; then
    apt-get download libgmp-dev=2:6.2.1+dfsg-3ubuntu1 libmpfr-dev=4.1.0-3build3
    for deb in ./*.deb; do dpkg-deb -x "$deb" ../ubuntu; done
fi
if [[ ! -x ../ubuntu/usr/bin/m4 ]]; then
    apt-get download m4
    for deb in ./m4_*.deb; do dpkg-deb -x "$deb" ../ubuntu; done
fi
export PATH="$root/deps/ubuntu/usr/bin:$PATH"
cp ../ubuntu/usr/include/x86_64-linux-gnu/gmp.h ../prefix/include/
cp ../ubuntu/usr/include/mpfr.h ../prefix/include/
# Reuse the installed matching runtime libraries through local symlinks.
ln -sf /lib/x86_64-linux-gnu/libgmp.so.10 ../prefix/lib/libgmp.so
ln -sf /lib/x86_64-linux-gnu/libmpfr.so.6 ../prefix/lib/libmpfr.so
if [[ ! -d ../flint-3.6.0 ]]; then tar -xzf flint-3.6.0.tar.gz -C ..; fi
cd ../flint-3.6.0
./configure --prefix="$root/deps/prefix" --with-gmp="$root/deps/prefix" \
    --with-mpfr="$root/deps/prefix" --disable-static --enable-shared \
    CFLAGS='-O3 -march=native' > "$root/build/configure.log" 2>&1
make -j4 > "$root/build/flint-build.log" 2>&1
make install > "$root/build/flint-install.log" 2>&1
sha256sum "$root/deps/downloads/"* > "$root/build/download-sha256.txt"
printf 'FLINT 3.6.0 local build complete: %s\n' "$root/deps/prefix"
