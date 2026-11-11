#!/bin/sh
# A release's tag names the release commit release-commit.sh makes, and a
# build of it takes timber's version from its tree alone (record 0031;
# rule core.74). release-commit.sh, run in scratch repositories, makes one
# commit wherever it runs: two clones of one commit, one of them under a
# host's settings that would sign, hook and name another author, make one
# id, the bot's, dated at its parent's commit time, changing exactly the
# files set-version.sh lists. It refuses a tree that differs from its
# commit and a version that is not major.minor.patch, moving nothing. It
# needs git and jq, the shell's, and no credential (core.26, core.38).
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

[ "$status" -ne 0 ] || echo "release: release-commit.sh makes one release commit wherever it runs"
exit $status
