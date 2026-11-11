#!/bin/sh
# Every file of the tree that carries timber's version gives the
# workspace's, which each crate takes, and the workspace's is 0.0.0: main's
# tree names no release. A release's tag names a release commit in which
# set-version.sh wrote the release's version into each of them
# (release-commit.sh; record 0031; rule core.74), so a build of the tag
# takes its version from the tree alone and a release's plugins carry its
# version, never drifting from its binary's. The files are the four this
# check reads, which `set-version.sh --files` lists.
#
# And set-version.sh, run on a scratch copy of those files with a version
# no release has, leaves each giving that version and changes no line but
# the ones that give it; refuses a version that is not major.minor.patch,
# and a file with no version, naming it, and writes nothing then.
set -u
root=$(cd "$(dirname "$0")" && pwd)
cd "$root" || exit 1

# The files this check reads.
files='Cargo.toml
Cargo.lock
plugins/claude-code/.claude-plugin/plugin.json
plugins/herdr/herdr-plugin.toml'

status=0

# set-version.sh lists exactly these files.
listed=$(sh set-version.sh --files) || {
  echo "versions: set-version.sh --files failed" >&2
  exit 1
}
if [ "$(printf '%s\n' "$listed" | sort)" != "$(printf '%s\n' "$files" | sort)" ]; then
  echo "versions: set-version.sh --files lists" >&2
  printf '%s\n' "$listed" | sed 's/^/  /' >&2
  echo "and this check reads" >&2
  printf '%s\n' "$files" | sed 's/^/  /' >&2
  status=1
fi

# The workspace's version in the tree at $1.
workspace() {
  awk '
    /^\[/ { section = $0 }
    section == "[workspace.package]" && /^version = "/ {
      gsub(/^version = "|"$/, "")
      print
      exit
    }
  ' "$1/Cargo.toml"
}

# Each place the tree at $1 gives timber's version, a line of where and the
# version it gives, "none" where it gives none.
carried() {
  for file in $files; do
    case $file in
    Cargo.toml)
      version=$(workspace "$1")
      printf '%s\t%s\n' "$file" "${version:-none}"
      ;;
    # timber's own crates are the entries of the lock with no source.
    Cargo.lock)
      awk '
          function flush() {
            if (package && !sourced) print "Cargo.lock: " name "\t" (version == "" ? "none" : version)
            package = 0
            sourced = 0
            name = ""
            version = ""
          }
          /^\[\[package\]\]$/ { flush(); package = 1; next }
          package && /^name = "/ { name = $0; gsub(/^name = "|"$/, "", name) }
          package && /^version = "/ { version = $0; gsub(/^version = "|"$/, "", version) }
          package && /^source = / { sourced = 1 }
          END { flush() }
        ' "$1/$file"
      ;;
    */plugin.json)
      jq -r --arg file "$file" '"\($file)\t\(.version // "none")"' "$1/$file"
      ;;
    */herdr-plugin.toml)
      awk -v file="$file" '
          /^\[/ { exit }
          /^version = "/ { version = $0; gsub(/^version = "|"$/, "", version) }
          END { print file "\t" (version == "" ? "none" : version) }
        ' "$1/$file"
      ;;
    esac
  done
}

# Each place in the tree at $1 that does not give the version $2, said with
# $3 ahead of it; fails where there is one.
agree() {
  carried "$1" | awk -F '\t' -v want="$2" -v said="$3" '
    $2 != want {
      print said $1 " gives " $2 ", and the workspace " want > "/dev/stderr"
      failed = 1
    }
    END { exit failed }
  '
}

version=$(workspace .)
if [ "$version" != 0.0.0 ]; then
  echo "versions: Cargo.toml gives ${version:-no version} under [workspace.package]; main's tree names no release, and gives 0.0.0" >&2
  exit 1
fi
agree . "$version" "versions: " || status=1

scratch=$(mktemp -d "${SLUICE_TMP:-${TMPDIR:-/tmp}}/timber-versions.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
copy() {
  rm -rf "$scratch/tree"
  for file in $files; do
    mkdir -p "$scratch/tree/$(dirname "$file")"
    cp "$file" "$scratch/tree/$file"
  done
}
# Whether the copy still holds the tree's files as they are; says which
# differs, after $1.
unchanged() {
  for file in $files; do
    cmp -s "$file" "$scratch/tree/$file" || {
      echo "versions: $1 changed $file" >&2
      return 1
    }
  done
}

# The agreement reports a manifest at another version, naming it and the
# version it gives.
copy
herdr=plugins/herdr/herdr-plugin.toml
sed 's/^version = ".*"$/version = "0.1.0"/' "$herdr" >"$scratch/tree/$herdr"
if agree "$scratch/tree" "$version" "" 2>"$scratch/said"; then
  echo "versions: a copy whose $herdr gives 0.1.0 passed the agreement" >&2
  status=1
elif ! grep -Fq "$herdr gives 0.1.0" "$scratch/said"; then
  echo "versions: a copy whose $herdr gives 0.1.0 failed without naming it and 0.1.0:" >&2
  cat "$scratch/said" >&2
  status=1
fi

# The release's writer on a scratch copy.
copy
release=98.76.54
if ! sh set-version.sh "$release" "$scratch/tree"; then
  echo "versions: set-version.sh $release failed" >&2
  exit 1
fi
agree "$scratch/tree" "$release" "versions: set-version.sh $release left " || status=1
for file in $files; do
  # A line it changed gives the version; it changes as many as it writes.
  changes=$(diff "$file" "$scratch/tree/$file")
  removed=$(printf '%s\n' "$changes" | grep -c '^<')
  added=$(printf '%s\n' "$changes" | grep -c '^>')
  stray=$(printf '%s\n' "$changes" | grep '^>' | grep -vF "\"$release\"")
  if [ "$added" -eq 0 ] || [ "$removed" != "$added" ] || [ -n "$stray" ]; then
    echo "versions: set-version.sh $release changed $file otherwise than its versions:" >&2
    printf '%s\n' "$changes" >&2
    status=1
  fi
done

# A version that is not major.minor.patch is refused, and nothing written.
copy
if sh set-version.sh 0.56 "$scratch/tree" 2>"$scratch/said"; then
  echo "versions: set-version.sh 0.56 did not refuse the version" >&2
  status=1
elif ! grep -Fq '0.56 is not a version major.minor.patch' "$scratch/said"; then
  echo "versions: set-version.sh 0.56 refused without saying 0.56 is not a version:" >&2
  cat "$scratch/said" >&2
  status=1
fi
unchanged "set-version.sh 0.56" || status=1

# A file with no version is refused, naming it, and nothing written.
copy
claude=plugins/claude-code/.claude-plugin/plugin.json
grep -v '^  "version": ' "$claude" >"$scratch/tree/$claude"
cp "$scratch/tree/$claude" "$scratch/stripped"
if sh set-version.sh "$release" "$scratch/tree" 2>"$scratch/said"; then
  echo "versions: set-version.sh $release did not refuse $claude with no version" >&2
  status=1
elif ! grep -Fq "$claude" "$scratch/said"; then
  echo "versions: set-version.sh $release refused $claude with no version without naming it:" >&2
  cat "$scratch/said" >&2
  status=1
fi
for file in $files; do
  [ "$file" = "$claude" ] && want=$scratch/stripped || want=$file
  cmp -s "$want" "$scratch/tree/$file" || {
    echo "versions: set-version.sh $release changed $file though it refused $claude" >&2
    status=1
  }
done
left=$(find "$scratch/tree" -name '*.set-version')
if [ -n "$left" ]; then
  echo "versions: set-version.sh $release left its scratch files behind: $left" >&2
  status=1
fi

[ "$status" -ne 0 ] || echo "versions: every file gives $version, and set-version.sh writes each"
exit $status
