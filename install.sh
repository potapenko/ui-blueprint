#!/bin/sh
# Release template. Run the rendered install.sh from a reviewed GitHub release.
set -eu
umask 022
VERSION='@VERSION@'
MIN_MACOS='@MIN_MACOS@'
ARCHIVE_SHA256='@ARCHIVE_SHA256@'
RELEASES='https://github.com/potapenko/ui-blueprint/releases/download'
FILES='@FILES@'
PREFIX=${HOME:?HOME must be set}/.local
SELECTED=$VERSION
UNINSTALL=false
STAGE=''
LOCK=''
NEW_LINKS=''

die() { printf 'uiblueprint: %s\n' "$*" >&2; exit 1; }
exists() { [ -e "$1" ] || [ -L "$1" ]; }
version_ok() { printf '%s\n' "$1" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+(-[A-Za-z0-9]+([.-][A-Za-z0-9]+)*)?$'; }
cleanup() {
  # STAGE contains only run-owned downloads and an admitted flat text/binary set.
  if [ -n "$STAGE" ] && [ -d "$STAGE" ]; then
    find "$STAGE" -type f ! -iname '*.png' ! -iname '*.jpg' ! -iname '*.jpeg' \
      ! -iname '*.gif' ! -iname '*.webp' ! -iname '*.svg' ! -iname '*.ico' \
      ! -iname '*.icns' ! -iname '*.bmp' ! -iname '*.tif' ! -iname '*.tiff' \
      ! -iname '*.heic' ! -iname '*.heif' ! -iname '*.avif' ! -iname '*.pdf' \
      ! -iname '*.partial' -exec rm -f {} +
    find "$STAGE" -depth -type d -exec sh -c 'rmdir "$1" 2>/dev/null || :' sh {} \;
  fi
  if [ -n "$NEW_LINKS" ]; then
    for command in $NEW_LINKS; do
      [ "$(readlink "$BIN/$command" 2>/dev/null || :)" != "$ROOT/current/$command" ] || rm "$BIN/$command"
    done
  fi
  if [ -n "$LOCK" ]; then
    rm -f "$LOCK/next"
    rmdir "$LOCK" 2>/dev/null || :
  fi
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM HUP

while [ "$#" -gt 0 ]; do
  case "$1" in
    --prefix) [ "$#" -ge 2 ] || die '--prefix needs a directory'; PREFIX=$2; shift 2 ;;
    --version) [ "$#" -ge 2 ] || die '--version needs a release version'; SELECTED=${2#v}; shift 2 ;;
    --uninstall) UNINSTALL=true; shift ;;
    --help|-h)
      printf '%s\n' 'Usage: sh install.sh [--prefix ABSOLUTE_DIRECTORY] [--version VERSION] [--uninstall]' \
        'Default prefix: ~/.local. No sudo or shell-profile changes.' \
        'Stop active sessions before update, rollback or removal.'
      exit 0 ;;
    *) die "unknown argument: $1" ;;
  esac
done
case "$PREFIX" in /*) ;; *) die '--prefix must be absolute' ;; esac
case "$PREFIX" in *"
"*|*"$(printf '\r')"*) die 'control characters in prefix' ;; esac
PREFIX=${PREFIX%/}
[ -n "$PREFIX" ] && [ "$PREFIX" != /usr ] && [ "$PREFIX" != /opt/homebrew ] || die 'choose a user-owned prefix'
ROOT=$PREFIX/share/uiblueprint
BIN=$PREFIX/bin
version_ok "$VERSION" || die 'unrendered installer; use install.sh from a release'
version_ok "$SELECTED" || die 'invalid release version'
[ "$(uname -s)" = Darwin ] && [ "$(uname -m)" = arm64 ] || die 'only Apple Silicon macOS is supported'
ACTUAL=$(sw_vers -productVersion)
awk -v actual="$ACTUAL" -v minimum="$MIN_MACOS" 'BEGIN {
  split(actual,a,"."); split(minimum,b,".");
  for(i=1;i<=3;i++) { if(a[i]+0>b[i]+0) exit 0; if(a[i]+0<b[i]+0) exit 1 }
}' || die "requires macOS $MIN_MACOS or later (found $ACTUAL)"

download() {
  curl --fail --silent --show-error --location --proto '=https' --proto-redir '=https' \
    --connect-timeout 15 --max-time 300 --output "$2" "$1" || die 'download failed; active version unchanged'
}

if [ "$UNINSTALL" = false ] && [ "$SELECTED" != "$VERSION" ]; then
  STAGE=$(mktemp -d "${TMPDIR:-/tmp}/uiblueprint-install.XXXXXX")
  download "$RELEASES/v$SELECTED/install.sh" "$STAGE/install.sh"
  grep -Fxq "VERSION='$SELECTED'" "$STAGE/install.sh" || die 'downloaded installer version mismatch'
  sh "$STAGE/install.sh" --prefix "$PREFIX"
  exit $?
fi

# Reject symlink parents in the installation-owned suffix, including an existing prefix.
for directory in "$PREFIX" "$PREFIX/share" "$BIN" "$ROOT" "$ROOT/versions"; do
  [ ! -L "$directory" ] || die "symlink directory: $directory"
  [ ! -e "$directory" ] || [ -d "$directory" ] || die "not a directory: $directory"
done
if exists "$ROOT"; then
  [ -f "$ROOT/.owner" ] && [ ! -L "$ROOT/.owner" ] || die 'installation directory has no ownership marker'
  [ "$(cat "$ROOT/.owner")" = ui-blueprint-installer-v1 ] || die 'foreign ownership marker'
elif [ "$UNINSTALL" = true ]; then
  die 'no managed installation at this prefix'
else
  mkdir -p "$PREFIX/share" "$BIN"
  mkdir "$ROOT" || die 'installation directory already exists'
  printf '%s\n' ui-blueprint-installer-v1 > "$ROOT/.owner"
fi
mkdir "$ROOT/.lock" 2>/dev/null || die 'another installer is active; inspect a stale .lock only after it stops'
LOCK=$ROOT/.lock
mkdir -p "$ROOT/versions" "$BIN"
for command in uiblueprint uiblueprint-validate; do
  if exists "$BIN/$command"; then
    [ -L "$BIN/$command" ] && [ "$(readlink "$BIN/$command")" = "$ROOT/current/$command" ] \
      || die "foreign command: $BIN/$command"
  fi
done
if exists "$ROOT/current"; then
  [ -L "$ROOT/current" ] || die 'current is not a managed symlink'
  ACTIVE=$(readlink "$ROOT/current")
  case "$ACTIVE" in versions/*) ;; *) die 'foreign current symlink' ;; esac
  version_ok "${ACTIVE#versions/}" || die 'invalid current version'
  [ -d "$ROOT/$ACTIVE" ] && [ ! -L "$ROOT/$ACTIVE" ] || die 'current version is missing or a symlink'
fi

verify_bundle() {
  folder=$1
  receipt=$folder/.install-sha256
  [ -d "$folder" ] && [ ! -L "$folder" ] || die 'invalid version directory'
  [ -f "$receipt" ] && [ ! -L "$receipt" ] || die 'missing regular install receipt'
  expected=$(printf '%s\n' "$FILES" | wc -l | tr -d ' ')
  [ "$(wc -l < "$receipt" | tr -d ' ')" -eq "$expected" ] || die 'invalid receipt length'
  grep -Eqv '^[0-9a-f]{64}  [A-Za-z0-9._-]+$' "$receipt" && die 'invalid receipt entry'
  for file in $FILES; do
    [ -f "$folder/$file" ] && [ ! -L "$folder/$file" ] || die "missing regular file: $file"
    [ "$(awk -v name="$file" '$2 == name { count++ } END { print count+0 }' "$receipt")" -eq 1 ] || die 'invalid receipt inventory'
    case "$file" in
      uiblueprint|uiblueprint-validate|session-worker|native-host-helper) mode=755 ;;
      *) mode=644 ;;
    esac
    [ "$(stat -f '%Lp' "$folder/$file")" = "$mode" ] || die "changed permissions: $file"
  done
  [ "$(find "$folder" -mindepth 1 -maxdepth 1 | wc -l | tr -d ' ')" -eq "$((expected+1))" ] \
    || die 'unknown files in version directory; nothing removed'
  (cd "$folder" && shasum -a 256 -c .install-sha256 >/dev/null) || die 'modified installation; nothing removed'
  for file in uiblueprint uiblueprint-validate session-worker native-host-helper; do
    [ -x "$folder/$file" ] || die "non-executable file: $file"
  done
}

if [ "$UNINSTALL" = true ]; then
  # Complete validation precedes the first removal. Globs include hidden strangers.
  for folder in "$ROOT/versions/"* "$ROOT/versions/".[!.]* "$ROOT/versions/"..?*; do
    exists "$folder" || continue
    version_ok "${folder##*/}" || die 'unknown item in versions; nothing removed'
    verify_bundle "$folder"
  done
  for command in uiblueprint uiblueprint-validate; do
    [ ! -L "$BIN/$command" ] || rm "$BIN/$command"
  done
  [ ! -L "$ROOT/current" ] || rm "$ROOT/current"
  for folder in "$ROOT/versions/"*; do
    exists "$folder" || continue
    for file in $FILES; do rm "$folder/$file"; done
    rm "$folder/.install-sha256"
    rmdir "$folder"
  done
  rmdir "$ROOT/versions"
  rm "$ROOT/.owner"
  rmdir "$LOCK"; LOCK=''
  rmdir "$ROOT" 2>/dev/null || :
  printf '%s\n' 'Removed UI Blueprint. Unrelated files and shared prefix directories were preserved.'
  exit 0
fi

DESTINATION=$ROOT/versions/$VERSION
if exists "$DESTINATION"; then
  verify_bundle "$DESTINATION"
  # Same tag with different bytes is a conflict, never an in-place overwrite.
  EXPECTED='@FILE_HASHES@'
  [ "$(cat "$DESTINATION/.install-sha256")" = "$EXPECTED" ] || die 'this version already exists with different bytes'
else
  STAGE=$(mktemp -d "${TMPDIR:-/tmp}/uiblueprint-install.XXXXXX")
  ARCHIVE=uiblueprint-v$VERSION-aarch64-apple-darwin.tar.gz
  download "$RELEASES/v$VERSION/$ARCHIVE" "$STAGE/bundle.tar.gz"
  printf '%s  %s\n' "$ARCHIVE_SHA256" "$STAGE/bundle.tar.gz" | shasum -a 256 -c - >/dev/null \
    || die 'archive checksum mismatch; active version unchanged'
  tar -tzf "$STAGE/bundle.tar.gz" > "$STAGE/names"
  printf '%s\n' "$FILES" | LC_ALL=C sort > "$STAGE/expected"
  LC_ALL=C sort "$STAGE/names" > "$STAGE/sorted"
  cmp -s "$STAGE/expected" "$STAGE/sorted" || die 'unexpected archive paths'
  tar -tvzf "$STAGE/bundle.tar.gz" > "$STAGE/types"
  grep -qv '^-' "$STAGE/types" && die 'archive contains a non-regular entry'
  mkdir "$STAGE/bundle"
  tar -xzf "$STAGE/bundle.tar.gz" -C "$STAGE/bundle"
  printf '%s\n' '@FILE_HASHES@' > "$STAGE/bundle/.install-sha256"
  verify_bundle "$STAGE/bundle"
  mv "$STAGE/bundle" "$DESTINATION"
fi
for command in uiblueprint uiblueprint-validate; do
  if ! exists "$BIN/$command"; then
    ln -s "$ROOT/current/$command" "$BIN/$command"
    NEW_LINKS="$NEW_LINKS $command"
  fi
done
ln -s "versions/$VERSION" "$LOCK/next"
mv -fh "$LOCK/next" "$ROOT/current"
NEW_LINKS=''
printf 'Installed UI Blueprint %s\nBundle: %s\n' "$VERSION" "$DESTINATION"
case ":${PATH:-}:" in
  *":$BIN:"*) ;;
  *) printf 'Add this directory to PATH: %s\n' "$BIN"
     quoted_bin=$(printf '%s' "$BIN" | sed "s/'/'\\\\''/g")
     printf "For this shell: export PATH='%s':\"\$PATH\"\n" "$quoted_bin" ;;
esac
printf 'Check: "%s/uiblueprint" --help\n' "$BIN"
