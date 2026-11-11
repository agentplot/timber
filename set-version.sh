#!/bin/sh
# shellcheck disable=SC2016 # awk programs name their own $fields.
# Writes a release's version into every file of the tree that carries
# timber's version: the workspace's in Cargo.toml, which each of the five
# crates takes, and Cargo.lock's entries for timber's own crates; the
# Claude Code plugin's manifest; and the herdr plugin's manifest. main
# gives 0.0.0 in each. release-commit.sh runs this on the commit it
# releases and commits the files `--files` lists as the release commit,
# which the version, binaries and release jobs each make and the release's
# tag names (.github/workflows/ci.yml). It writes every file or none: a
# file it finds no version in is refused, naming it, before any file is
# moved. root:versions holds each file to the workspace's version, and
# runs this on a scratch copy of them (check-versions.sh). Record 0031;
# rule core.74.
#
#   sh set-version.sh <major.minor.patch> [<root>]
#   sh set-version.sh --files
set -eu

# The files that carry timber's version, from the root: the one list the
# writer walks, release-commit.sh commits and the checks read.
files='Cargo.toml
Cargo.lock
plugins/claude-code/.claude-plugin/plugin.json
plugins/herdr/herdr-plugin.toml'

usage() {
  echo "usage: set-version.sh <major.minor.patch> [<root>] | --files" >&2
  exit 2
}

[ $# -ge 1 ] && [ $# -le 2 ] || usage
if [ "$1" = --files ]; then
  [ $# -eq 1 ] || usage
  printf '%s\n' "$files"
  exit 0
fi
version=$1
root=${2:-$(dirname "$0")}
printf '%s\n' "$version" | grep -Eqx '[0-9]+\.[0-9]+\.[0-9]+' || {
  echo "set-version: $version is not a version major.minor.patch" >&2
  exit 2
}

# The awk program that writes the version `v` into the file $1, counting
# what it wrote in `written`.
program() {
  case $1 in
  Cargo.toml)
    echo '
      /^\[/ { section = $0 }
      section == "[workspace.package]" && /^version = "/ && !written {
        $0 = "version = \"" v "\""
        written++
      }
      { print }
    '
    ;;
  # timber's own crates are the entries of the lock with no source.
  Cargo.lock)
    echo '
      function flush(i) {
        for (i = 1; i <= n; i++) {
          if (package && !sourced && lines[i] ~ /^version = "/) {
            lines[i] = "version = \"" v "\""
            written++
          }
          print lines[i]
        }
        n = 0
        sourced = 0
      }
      /^\[\[package\]\]$/ { flush(); package = 1 }
      /^source = / { sourced = 1 }
      { lines[++n] = $0 }
      END { flush() }
    '
    ;;
  # The manifest's own version, at the object's top level.
  */plugin.json)
    echo '
      /^  "version": "[^"]*",?$/ && !written {
        sub(/"version": "[^"]*"/, "\"version\": \"" v "\"")
        written++
      }
      { print }
    '
    ;;
  # The manifest's own version, ahead of its first table.
  */herdr-plugin.toml)
    echo '
      /^\[/ { tables = 1 }
      !tables && /^version = "/ && !written {
        $0 = "version = \"" v "\""
        written++
      }
      { print }
    '
    ;;
  *)
    echo "set-version: no program writes $1's version" >&2
    return 1
    ;;
  esac
}

# Each file is written beside itself as <file>.set-version; only once every
# file has been written is any moved over its original.
refused=0
for file in $files; do
  path=$root/$file
  if [ ! -f "$path" ]; then
    echo "set-version: $file is not under $root" >&2
    refused=1
    continue
  fi
  awk -v v="$version" "$(program "$file")"'
    END { exit (written ? 0 : 1) }
  ' "$path" >"$path.set-version" || {
    rm -f "$path.set-version"
    echo "set-version: $file gives no version where timber's is" >&2
    refused=1
  }
done
if [ "$refused" -ne 0 ]; then
  for file in $files; do
    rm -f "$root/$file.set-version"
  done
  exit 1
fi
for file in $files; do
  mv "$root/$file.set-version" "$root/$file"
done
