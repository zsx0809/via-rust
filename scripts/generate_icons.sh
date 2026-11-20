#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$DIR"

echo "=== Generating Android Launcher Icons ==="

declare -A LEGACY_SIZES=(
  ["mipmap-mdpi"]=48
  ["mipmap-hdpi"]=72
  ["mipmap-xhdpi"]=96
  ["mipmap-xxhdpi"]=144
  ["mipmap-xxxhdpi"]=192
)

declare -A FG_SIZES=(
  ["mipmap-mdpi"]=108
  ["mipmap-hdpi"]=162
  ["mipmap-xhdpi"]=216
  ["mipmap-xxhdpi"]=324
  ["mipmap-xxxhdpi"]=432
)

for DENSITY in "${!LEGACY_SIZES[@]}"; do
  SIZE=${LEGACY_SIZES[$DENSITY]}
  mkdir -p "res/$DENSITY"
  rsvg-convert -w "$SIZE" -h "$SIZE" ui/icon_legacy.svg -o "res/$DENSITY/ic_launcher.png"
  echo "Generated res/$DENSITY/ic_launcher.png (${SIZE}x${SIZE})"
done

for DENSITY in "${!FG_SIZES[@]}"; do
  SIZE=${FG_SIZES[$DENSITY]}
  mkdir -p "res/$DENSITY"
  rsvg-convert -w "$SIZE" -h "$SIZE" ui/icon_foreground.svg -o "res/$DENSITY/ic_launcher_foreground.png"
  echo "Generated res/$DENSITY/ic_launcher_foreground.png (${SIZE}x${SIZE})"
done

echo "=== All Icons Successfully Generated ==="
