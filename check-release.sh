#!/bin/sh
# shellcheck disable=SC2016 # the workflow's lines, read as written.
# A release's tag names the release commit release-commit.sh makes, and a
# build of it takes timber's version from its tree alone (record 0031;
# rule core.74). release-commit.sh, run in scratch repositories, makes one
# commit wherever it runs: two clones of one commit, one of them under a
# host's settings that would sign, hook and name another author, make one
# id, the bot's, dated at its parent's commit time, changing exactly the
# files set-version.sh lists. It refuses a tree that differs from its
# commit and a version that is not major.minor.patch, moving nothing. The
# flake versions its package by Cargo.toml's [workspace.package] alone.
# And CI's workflow releases from the release commit: the version job
# reckons with --unreleased, makes the commit and hands on its id; the
# binaries job makes it again, holds it to that id and builds it with
# nothing given; the release job does too, tags it, pushes the tag alone
# and publishes on it; the build job holds main to 0.0.0; and the
# conventional job starts at the newest tag by version. It needs git and
# jq, the shell's, and no credential (core.26, core.38).
set -u
root=$(cd "$(dirname "$0")" && pwd)
cd "$root" || exit 1

status=0
fail() {
  echo "release: $*" >&2
  status=1
}

scratch=$(mktemp -d "${SLUICE_TMP:-${TMPDIR:-/tmp}}/timber-release.XXXXXX")
trap 'rm -rf "$scratch"' EXIT

# git as a scratch repository's, under no host's settings but the ones a
# case gives.
GIT_CONFIG_GLOBAL=$scratch/empty.gitconfig
GIT_CONFIG_NOSYSTEM=1
export GIT_CONFIG_GLOBAL GIT_CONFIG_NOSYSTEM
: >"$GIT_CONFIG_GLOBAL"

files=$(sh set-version.sh --files) || {
  echo "release: set-version.sh --files failed" >&2
  exit 1
}

# The scratch commit: the writer, the release's script and the files that
# carry timber's version, committed with a fixed identity and date.
origin=$scratch/origin
git init -q -b main "$origin"
for file in set-version.sh release-commit.sh $files; do
  mkdir -p "$origin/$(dirname "$file")"
  cp "$file" "$origin/$file"
done
git -C "$origin" add -A
GIT_AUTHOR_NAME=scratch GIT_AUTHOR_EMAIL=scratch@example.invalid \
  GIT_AUTHOR_DATE='@1700000000 +0000' \
  GIT_COMMITTER_NAME=scratch GIT_COMMITTER_EMAIL=scratch@example.invalid \
  GIT_COMMITTER_DATE='@1712345678 +0000' \
  git -C "$origin" commit -q -m 'chore: the scratch commit'
base=$(git -C "$origin" rev-parse HEAD)
when=$(git -C "$origin" log -1 --format=%ct HEAD)

clone() {
  git clone -q "$origin" "$scratch/$1"
}

# A host's settings that would sign the commit, run its hooks and name
# another author; release-commit.sh takes none of them.
mkdir -p "$scratch/hooks"
for hook in pre-commit prepare-commit-msg commit-msg; do
  cat >"$scratch/hooks/$hook" <<'EOF'
#!/bin/sh
[ -z "${1:-}" ] || echo "a hook wrote this" >>"$1"
EOF
  chmod +x "$scratch/hooks/$hook"
done
cat >"$scratch/host.gitconfig" <<EOF
[user]
  name = a host
  email = host@example.invalid
[commit]
  gpgsign = true
[gpg]
  program = false
[core]
  hooksPath = $scratch/hooks
EOF

bot='github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>'
release=98.76.54
clone a
clone b
if ! sh "$scratch/a/release-commit.sh" "$release" >/dev/null 2>"$scratch/said"; then
  fail "release-commit.sh $release failed in a clone:"
  cat "$scratch/said" >&2
fi
if ! GIT_CONFIG_GLOBAL=$scratch/host.gitconfig \
  sh "$scratch/b/release-commit.sh" "$release" >/dev/null 2>"$scratch/said"; then
  fail "release-commit.sh $release failed in a clone under a host's settings:"
  cat "$scratch/said" >&2
fi
a=$(git -C "$scratch/a" rev-parse HEAD)
b=$(git -C "$scratch/b" rev-parse HEAD)
[ "$a" = "$b" ] || fail "release-commit.sh $release made $a in one clone and $b in another"
[ "$a" != "$base" ] || fail "release-commit.sh $release made no commit"

# The commit: its subject, its one parent, the bot and its parent's time.
subject=$(git -C "$scratch/a" log -1 --format=%B HEAD)
[ "$subject" = "chore(release): v$release" ] ||
  fail "the release commit says '$subject', not 'chore(release): v$release'"
parents=$(git -C "$scratch/a" log -1 --format=%P HEAD)
[ "$parents" = "$base" ] ||
  fail "the release commit's parents are '$parents', not the scratch commit $base"
for who in a c; do
  name=$(git -C "$scratch/a" log -1 --format="%${who}n <%${who}e>" HEAD)
  [ "$name" = "$bot" ] || fail "the release commit's %${who}n is $name, not $bot"
  at=$(git -C "$scratch/a" log -1 --format="%${who}t" HEAD)
  [ "$at" = "$when" ] || fail "the release commit's %${who}t is $at, not its parent's commit time $when"
done
signed=$(git -C "$scratch/b" cat-file commit HEAD | grep -c '^gpgsig') || :
[ "$signed" = 0 ] || fail "the release commit under a host's settings is signed"

# It changes exactly the files set-version.sh lists, each to the version.
changed=$(git -C "$scratch/a" diff --name-only HEAD^ HEAD | sort)
[ "$changed" = "$(printf '%s\n' "$files" | sort)" ] || {
  fail "the release commit changes"
  printf '%s\n' "$changed" | sed 's/^/  /' >&2
  echo "and set-version.sh lists" >&2
  printf '%s\n' "$files" | sed 's/^/  /' >&2
}
git -C "$scratch/a" show HEAD:Cargo.toml | grep -qx "version = \"$release\"" ||
  fail "the release commit's Cargo.toml gives no version $release"

# A tree that differs from its commit is refused, and HEAD stays.
clone c
echo '# changed' >>"$scratch/c/Cargo.toml"
if sh "$scratch/c/release-commit.sh" "$release" >/dev/null 2>"$scratch/said"; then
  fail "release-commit.sh $release did not refuse a tree that differs from its commit"
elif ! grep -Fq "release-commit: the tree differs from its commit $(git -C "$scratch/c" rev-parse --short HEAD)" "$scratch/said"; then
  fail "release-commit.sh $release refused a changed tree without saying it differs from its commit:"
  cat "$scratch/said" >&2
fi
[ "$(git -C "$scratch/c" rev-parse HEAD)" = "$base" ] ||
  fail "release-commit.sh $release moved HEAD in a tree that differs from its commit"

# A version that is not major.minor.patch is refused, and nothing moves.
clone d
if sh "$scratch/d/release-commit.sh" 0.56 >/dev/null 2>"$scratch/said"; then
  fail "release-commit.sh 0.56 did not refuse the version"
fi
[ "$(git -C "$scratch/d" rev-parse HEAD)" = "$base" ] ||
  fail "release-commit.sh 0.56 moved HEAD"
[ -z "$(git -C "$scratch/d" status --porcelain)" ] ||
  fail "release-commit.sh 0.56 left the tree changed: $(git -C "$scratch/d" status --porcelain)"

# The flake versions its package by Cargo.toml's [workspace.package] alone:
# no variable, no impure read.
if grep -n 'TIMBER_VERSION\|getEnv' flake.nix >"$scratch/said"; then
  fail "flake.nix reads a version from outside the tree:"
  cat "$scratch/said" >&2
fi
grep -q 'workspace = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package;' flake.nix ||
  fail "flake.nix reads no workspace from Cargo.toml's [workspace.package]"
grep -q '^ *inherit (workspace) version;$' flake.nix ||
  fail "flake.nix versions its package by something other than the workspace's version"

# CI releases from the release commit. Each job of the workflow is read as
# its lines from `  <job>:` to the next job's, its comments left out.
ci=.github/workflows/ci.yml
job() {
  awk -v job="  $1:" '
    $0 == job { inside = 1; next }
    inside && /^  [A-Za-z0-9_-]+:$/ { exit }
    inside && !/^ *#/ { print }
  ' "$ci"
}
# Whether the job $1 has a line holding the text $2; fails saying $3.
names() {
  job "$1" | grep -Fq -- "$2" || fail "$ci's $1 job $3: no line holds $2"
}
# Whether the job $1 has no line holding the text $2; fails saying $3.
lacks() {
  if job "$1" | grep -Fn -- "$2" >"$scratch/said"; then
    fail "$ci's $1 job $3:"
    cat "$scratch/said" >&2
  fi
}
for each in build conventional version binaries release; do
  [ -n "$(job "$each")" ] || fail "$ci has no $each job"
done

# Nothing hands a build a version.
if grep -n 'TIMBER_VERSION' "$ci" >"$scratch/said"; then
  fail "$ci names TIMBER_VERSION:"
  cat "$scratch/said" >&2
fi
if grep -n 'nix build' "$ci" | grep -F -- '--impure' >"$scratch/said"; then
  fail "$ci builds with --impure:"
  cat "$scratch/said" >&2
fi

# build: main's package and binary say 0.0.0, and the binary its commit.
names build 'version="$(nix eval --raw .#default.version)"' "reads no package version"
names build '[ "$version" = 0.0.0 ]' "holds the package to no 0.0.0"
names build 'nix build .#default' "builds no package"
names build '"timber 0.0.0, revision $(git rev-parse HEAD), built by "*)' \
  "matches timber --version to no 0.0.0 at the commit built"

# conventional: from the newest tag by version.
names conventional "git tag --list 'v*' --sort=-v:refname" "starts at no newest tag by version"
lacks conventional 'git describe' "starts at git describe's tag, on main's history"

# version: reckons with --unreleased, makes the release commit and hands
# on its id.
names version 'git-cliff --unreleased --bumped-version' "reckons the version otherwise than from the newest tag"
names version 'sh release-commit.sh "${tag#v}"' "makes no release commit"
names version 'tag: ${{ steps.next.outputs.tag }}' "hands on no tag"
names version 'commit: ${{ steps.next.outputs.commit }}' "hands on no commit"
names version 'echo "commit=$(git rev-parse HEAD)" >> "$GITHUB_OUTPUT"' "hands on no release commit's id"

# binaries and release: the release commit made again, held to the
# version job's.
for each in binaries release; do
  names "$each" 'COMMIT: ${{ needs.version.outputs.commit }}' "is given no version job's commit"
  names "$each" 'sh release-commit.sh "${TAG#v}"' "makes no release commit"
  names "$each" 'made="$(git rev-parse HEAD)"' "reads no commit it made"
  names "$each" '[ "$made" = "$COMMIT" ]' "holds its commit to no version job's"
done

# binaries: the package's version is the release's, built with nothing
# given, and the binary says it and the commit.
names binaries 'version="$(nix eval --raw .#default.version)"' "reads no package version"
names binaries '[ "$version" = "${TAG#v}" ]' "holds the package to no release's version"
names binaries 'nix build .#default' "builds no package"
names binaries '"timber ${TAG#v}, revision $COMMIT, built by "*)' \
  "matches timber --version to no release's version at its commit"

# release: the notes, from main's range, then the commit, the tag, the tag
# pushed alone and the release on it.
names release 'git-cliff --unreleased' "writes no notes from the newest tag"
notes=$(job release | grep -nF 'git-cliff --unreleased' | head -n 1 | cut -d : -f 1)
made=$(job release | grep -nF 'sh release-commit.sh' | head -n 1 | cut -d : -f 1)
[ -z "$notes" ] || [ -z "$made" ] || [ "$notes" -lt "$made" ] ||
  fail "$ci's release job writes its notes after the release commit, whose range is not main's"
names release 'git tag "$TAG"' "tags nothing"
pushes=$(job release | grep -F 'git push' | sed 's/^ *//')
[ "$pushes" = 'git push origin "refs/tags/$TAG"' ] || {
  fail "$ci's release job pushes otherwise than the tag alone, once, with no + and no --force:"
  printf '%s\n' "$pushes" >&2
}
lacks release '--force' "forces a push"
names release 'gh release create "$TAG" --verify-tag' "publishes on no tag it verifies"
lacks release '--target' "publishes at a target, not the tag"

[ "$status" -ne 0 ] || echo "release: release-commit.sh makes one release commit wherever it runs, a build takes timber's version from its tree alone, and CI releases from the release commit"
exit $status
