#!/usr/bin/env bash
set -e
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path/.."

# Setup web
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.93

OPEN=false
FEATURES=""
OPTIMIZE=false
BUILD=debug
BUILD_FLAGS=""
WGPU=false
WASM_OPT_FLAGS="-O2 --fast-math"

while test $# -gt 0; do
  case "$1" in
    -h|--help)
      echo "build_web.sh [--release] [--webgpu] [--open] --crate CRATE --prefix PREFIX"
      echo ""
      echo "  --crate:   Crate to build."
      echo ""
      echo "  --prefix:    Output prefix."
      echo ""
      echo "  -g:        Keep debug symbols even with --release."
      echo "             These are useful profiling and size trimming."
      echo ""
      echo "  --open:    Open the result in a browser."
      echo ""
      echo "  --release: Build with --release, and then run wasm-opt."
      echo "             NOTE: --release also removes debug symbols, unless you also use -g."
      echo ""
      echo "  --wgpu:    Build a binary using wgpu instead of glow/webgl."
      echo "             The resulting binary will automatically use WebGPU if available and"
      echo "             fall back to a WebGL emulation layer otherwise."
      exit 0
      ;;

    --crate)
      CRATE=$2
      shift
      shift
      ;;

    --prefix)
      PREFIX=$2
      shift
      shift
      ;;

    -g)
      shift
      WASM_OPT_FLAGS="${WASM_OPT_FLAGS} -g"
      ;;

    --open)
      shift
      OPEN=true
      ;;

    --release)
      shift
      OPTIMIZE=true
      BUILD="release"
      BUILD_FLAGS="--release"
      ;;

    --wgpu)
      shift
      WGPU=true
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

if [[ -z $CRATE ]];  then echo "Crate must be specified with --crate."; exit 1; fi
if [[ -z $PREFIX ]]; then echo "Prefix must be specified with --prefix."; exit 1; fi
if [[ "$FEATURES" != "" ]]; then BUILD_FLAGS="${BUILD_FLAGS} --features $FEATURES"; fi

FINAL_WASM_PATH=site/${PREFIX}_bg.wasm

# Clear output from old stuff:
rm -f "${FINAL_WASM_PATH}"

echo "Building rust…"

(cd crates/$CRATE &&
  pwd &&
  cargo build \
    ${BUILD_FLAGS} \
    --lib \
    --target wasm32-unknown-unknown \
    --no-default-features
)

# Get the output directory (in the workspace it is in another location)
# TARGET=`cargo metadata --format-version=1 | jq --raw-output .target_directory`
TARGET="target"

echo "Generating JS bindings for wasm…"
TARGET_NAME="${CRATE}.wasm"
WASM_PATH="${TARGET}/wasm32-unknown-unknown/$BUILD/$TARGET_NAME"
wasm-bindgen "${WASM_PATH}" --out-dir site --out-name ${PREFIX} --no-modules --no-typescript

# if this fails with "error: cannot import from modules (`env`) with `--no-modules`", you can use:
# wasm2wat target/wasm32-unknown-unknown/release/pocket.wasm | rg env
# wasm2wat target/wasm32-unknown-unknown/release/pocket.wasm | rg "call .now\b" -B 20 # What calls `$now` (often a culprit)
# Or use https://rustwasm.github.io/twiggy/usage/command-line-interface/paths.html#twiggy-paths

# to get wasm-strip:  apt/brew/dnf install wabt
# wasm-strip ${FINAL_WASM_PATH}

if [[ "${OPTIMIZE}" = true ]]; then
  echo "Optimizing wasm…"
  # to get wasm-opt:  apt/brew/dnf install binaryen
  wasm-opt "${FINAL_WASM_PATH}" $WASM_OPT_FLAGS -o "${FINAL_WASM_PATH}"
fi

echo "Finished ${FINAL_WASM_PATH}"

if [[ "${OPEN}" == true ]]; then
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux, ex: Fedora
    xdg-open http://localhost:8765/index.html
  elif [[ "$OSTYPE" == "msys" ]]; then
    # Windows
    start http://localhost:8765/index.html
  else
    # Darwin/MacOS, or something else
    open http://localhost:8765/index.html
  fi
fi
