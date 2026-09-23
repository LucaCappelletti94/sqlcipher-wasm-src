#!/bin/sh -e

SQLCIPHER_VERSION="4.19.0"
SQLCIPHER_SHA256="7075f96cbabe45b4ecfc2e6b1745a625f856f695b0827a5506ce9ed85b906aa0"
LIBTOMCRYPT_VERSION="1.18.2"
LIBTOMCRYPT_SHA256="96ad4c3b8336050993c5bc2cf6c057484f2b0f9f763448151567fbab5e767b84"

cd "$(dirname "$0")"
ROOT=$(pwd)
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

cd "$WORK"
curl -sfL -o sqlcipher.tar.gz "https://github.com/sqlcipher/sqlcipher/archive/refs/tags/v${SQLCIPHER_VERSION}.tar.gz"
curl -sfL -o libtomcrypt.tar.xz "https://github.com/libtom/libtomcrypt/releases/download/v${LIBTOMCRYPT_VERSION}/crypt-${LIBTOMCRYPT_VERSION}.tar.xz"
printf '%s  sqlcipher.tar.gz\n%s  libtomcrypt.tar.xz\n' "$SQLCIPHER_SHA256" "$LIBTOMCRYPT_SHA256" | shasum -a 256 -c -

mkdir sqlcipher libtomcrypt
tar xzf sqlcipher.tar.gz --strip-components=1 -C sqlcipher
tar xJf libtomcrypt.tar.xz --strip-components=1 -C libtomcrypt
(cd sqlcipher && ./configure > configure.log && make sqlite3.c > make.log)

python3 "$ROOT/tools/assemble.py" "$WORK/sqlcipher" "$WORK/libtomcrypt" "$ROOT/sqlcipher"

echo "Now set the versions in src/lib.rs and Cargo.toml."
