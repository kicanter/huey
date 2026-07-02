#!/bin/sh

set -e
VERSION="${1:-latest}"
REPO="kicanter/libchroma"

# Detect OS + arch
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
    darwin) OS="macos" ;;
    linux)  OS="linux-gnu" ;;
    *)      echo "unsupported OS: $OS" >&2; exit 1 ;;
esac

case "$ARCH" in
    arm64|aarch64) ARCH="aarch64" ;;
    x86_64)        ARCH="x86_64" ;;
    *)             echo "unsupported arch: $ARCH" >&2; exit 1 ;;
esac

TARBALL="libchroma-${ARCH}-${OS}.tar.gz"

if [ "$VERSION" = "latest" ]; then
  VERSION=$(gh release list --repo "$REPO" --limit 1 --json tagName --jq '.[0].tagName')
fi

URL="https://github.com/$REPO/releases/download/$VERSION/$TARBALL"

mkdir -p vendor
curl -fL "$URL" | tar xz -C vendor --include='./libchroma.a' --include='./chroma.h'
echo "vendored libchroma $VERSION ($ARCH-$OS)"
