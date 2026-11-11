#!/bin/sh
# Makes the release commit of a version on the commit checked out: runs
# set-version.sh, and commits the files it lists by path as
# `chore(release): v<version>`, with the checked-out commit its one
# parent. CI's version, binaries and release jobs each make it on the
# pushed commit, and the release's tag names it (.github/workflows/ci.yml),
# so a build of the tag takes its version from the tree alone. It makes
# one id wherever it runs: the bot is its author and committer, both
# dated as its parent was committed, and no host's hook, signing or
# identity enters it. It runs from the root of the checkout it is in, and
# refuses a tree whose tracked files differ from their commit, committing
# nothing. root:release proves it in scratch repositories
# (check-release.sh). Record 0031; rule core.74.
#
#   sh release-commit.sh <major.minor.patch>
set -eu

[ $# -eq 1 ] || {
  echo "usage: release-commit.sh <major.minor.patch>" >&2
  exit 2
}
version=$1
cd "$(dirname "$0")"

git diff --quiet HEAD -- || {
  echo "release-commit: the tree differs from its commit $(git rev-parse --short HEAD)" >&2
  exit 1
}

sh ./set-version.sh "$version" .
files=$(sh ./set-version.sh --files)

bot='github-actions[bot]'
mail='41898282+github-actions[bot]@users.noreply.github.com'
when="@$(git log -1 --format=%ct HEAD) +0000"
# shellcheck disable=SC2086 # the files, one word each.
GIT_AUTHOR_NAME=$bot GIT_AUTHOR_EMAIL=$mail GIT_AUTHOR_DATE=$when \
  GIT_COMMITTER_NAME=$bot GIT_COMMITTER_EMAIL=$mail GIT_COMMITTER_DATE=$when \
  git -c commit.gpgsign=false -c core.hooksPath=/dev/null \
  commit --quiet --no-verify -m "chore(release): v$version" -- $files
echo "release-commit: chore(release): v$version at $(git rev-parse HEAD)"
