#!/bin/sh -e

SQLCIPHER_VERSION="4.19.0"
# A signed tag moved to another commit still fails.
SQLCIPHER_COMMIT="c4b275a47932888216bade83aff2bbc73df0ff85"
# Stephen Lombardo's key from keys.openpgp.org, in keys/sqlcipher.asc.
SQLCIPHER_SIGNER="D92204901CD8BFDF63A2D9F952E8883F1591F4CE"
LIBTOMCRYPT_VERSION="1.18.2"
LIBTOMCRYPT_SHA256="96ad4c3b8336050993c5bc2cf6c057484f2b0f9f763448151567fbab5e767b84"
# Steffen Jaeckel's key from keyserver.ubuntu.com, in keys/libtomcrypt.asc.
LIBTOMCRYPT_SIGNER="C4386A237ED43A475541B9427B2CD0DD4BCFF59B"

cd "$(dirname "$0")"
ROOT=$(pwd)
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
cd "$WORK"

# Only the committed keys count, never the machine's keyring.
GNUPGHOME="$WORK/gnupg"
export GNUPGHOME
mkdir -m 700 "$GNUPGHOME"
gpg --quiet --import "$ROOT"/keys/*.asc

# Accepts a valid signature by primary key $1 even from an expired key, as libtomcrypt 1.18.2's is, never a revoked one.
signed_by() {
    awk -v fpr="$1" '$1 == "[GNUPG:]" && $2 == "VALIDSIG" && $NF == fpr { ok = 1 }
        $1 == "[GNUPG:]" && $2 == "REVKEYSIG" { revoked = 1 }
        END { exit !(ok && !revoked) }'
}

git init --quiet sqlcipher.git
git -C sqlcipher.git fetch --quiet --depth 1 https://github.com/sqlcipher/sqlcipher.git \
    "refs/tags/v${SQLCIPHER_VERSION}:refs/tags/v${SQLCIPHER_VERSION}"
git -C sqlcipher.git verify-tag --raw "v${SQLCIPHER_VERSION}" 2>&1 | signed_by "$SQLCIPHER_SIGNER" ||
    { echo "SQLCipher v${SQLCIPHER_VERSION} carries no valid signature from ${SQLCIPHER_SIGNER}" >&2; exit 1; }
TAGGED=$(git -C sqlcipher.git rev-parse "v${SQLCIPHER_VERSION}^{commit}")
[ "$TAGGED" = "$SQLCIPHER_COMMIT" ] ||
    { echo "SQLCipher v${SQLCIPHER_VERSION} points at ${TAGGED}, not ${SQLCIPHER_COMMIT}" >&2; exit 1; }

RELEASE="https://github.com/libtom/libtomcrypt/releases/download/v${LIBTOMCRYPT_VERSION}/crypt-${LIBTOMCRYPT_VERSION}.tar.xz"
curl -sfL -o libtomcrypt.tar.xz "$RELEASE"
curl -sfL -o libtomcrypt.tar.xz.asc "$RELEASE.asc"
gpg --status-fd 1 --verify libtomcrypt.tar.xz.asc libtomcrypt.tar.xz 2>/dev/null | signed_by "$LIBTOMCRYPT_SIGNER" ||
    { echo "libtomcrypt ${LIBTOMCRYPT_VERSION} carries no valid signature from ${LIBTOMCRYPT_SIGNER}" >&2; exit 1; }
echo "$LIBTOMCRYPT_SHA256  libtomcrypt.tar.xz" | shasum -a 256 -c -

mkdir sqlcipher libtomcrypt
git -C sqlcipher.git archive "$SQLCIPHER_COMMIT" | tar x -C sqlcipher
tar xJf libtomcrypt.tar.xz --strip-components=1 -C libtomcrypt
(cd sqlcipher && ./configure > configure.log && make sqlite3.c > make.log)

python3 "$ROOT/tools/assemble.py" "$WORK/sqlcipher" "$WORK/libtomcrypt" "$ROOT/sqlcipher"
