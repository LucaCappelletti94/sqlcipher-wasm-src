#!/bin/sh -e

cd "$(dirname "$0")"
ROOT=$(pwd)
# shellcheck source=tools/releases.sh
. "$ROOT/tools/releases.sh"
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

trust_keys "$WORK"
fetch_sqlcipher "$WORK"
fetch_libtomcrypt "$WORK"
(cd "$WORK/sqlcipher" && ./configure > configure.log && make sqlite3.c > make.log)

python3 "$ROOT/tools/assemble.py" "$WORK/sqlcipher" "$WORK/libtomcrypt" "$ROOT/sqlcipher"

# sqlite-wasm-rs's own bindgen setup, plus the define SQLCipher's header puts sqlite3_key behind.
LIBCLANG_PATH="/usr/lib/llvm-${LLVM_MAJOR}/lib"
export LIBCLANG_PATH
[ -f "$LIBCLANG_PATH/libclang-${LLVM_MAJOR}.so" ] || { echo "libclang ${LLVM_MAJOR} not found in $LIBCLANG_PATH" >&2; exit 1; }
out=$(BINDGEN_EXTRA_CLANG_ARGS=-DSQLITE_HAS_CODEC SQLITE_WASM_RS_SOURCE_DIR="$ROOT/sqlcipher" CARGO_TARGET_DIR="$WORK/target" \
    cargo build --quiet --locked --manifest-path "$ROOT/interop/web/Cargo.toml" --lib \
    --target wasm32-unknown-unknown --features sqlite-wasm-rs/bindgen --message-format json |
    jq -r 'select(.reason == "build-script-executed" and (.package_id | test("sqlite-wasm-rs"))) | .out_dir')
[ -f "$out/bindgen.rs" ] || { echo "sqlite-wasm-rs produced no bindings" >&2; exit 1; }
cp "$out/bindgen.rs" "$ROOT/sqlcipher/sqlcipher_bindgen.rs"

python3 "$ROOT/tools/checksums.py" "$ROOT/sqlcipher"
