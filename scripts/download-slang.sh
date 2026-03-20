#!/usr/bin/env bash
{
VERSION="${1:-}"
PLATFORM="${2:-}"

if [ -z "$PLATFORM" ]; then
  echo "Usage: $0 <version> <platform>"
  exit 1
fi

ASSET_NAME="slang-${VERSION}-${PLATFORM}.zip"
URL="https://github.com/shader-slang/slang/releases/download/v${VERSION}/${ASSET_NAME}"
echo "Downloading Slang ${VERSION} for ${PLATFORM}"
echo "URL: ${URL}"

curl -L -o "slang-release.zip" "$URL"

echo "Extracting..."
unzip -q slang-release.zip -d slang_dir
rm -rf slang-release.zip

SLANG_DIR=$(cd slang_dir && pwd)

# windows needs to use cygpath to get proper separators.
if command -v cygpath &> /dev/null; then
  SLANG_DIR=$(cygpath -w "$SLANG_DIR")
fi

echo "Extracted to: ${SLANG_DIR}"
} >&2
SLANG_DIR=$(cd slang_dir && pwd)

# windows needs to use cygpath to get proper separators.
if command -v cygpath &> /dev/null; then
  SLANG_DIR=$(cygpath -w "$SLANG_DIR")
fi

echo "SLANG_DIR=${SLANG_DIR}"
echo "LD_LIBRARY_PATH=${SLANG_DIR}/lib:${LD_LIBRARY_PATH}"
echo "DYLD_LIBRARY_PATH=${SLANG_DIR}/lib:${DYLD_LIBRARY_PATH}"
