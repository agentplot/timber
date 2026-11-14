#!/bin/sh
# The modules' activation, run as the user who runs timber (record 0035,
# core.76):
#
#   activate.sh link <file>
#
# links ~/.config/timber/config.toml to the module's file, so every
# timber command that user runs, and the serving process at each request,
# reads it. A link there is replaced. A file or folder there that is no
# link is moved once, to config.toml.before-nix, and both paths are named.
# Where config.toml.before-nix is there already, it changes neither, names
# both and fails. It makes ~/.config/timber where it is missing and links
# the file alone, never the folder, which may hold a host key the file
# names. It says a line only where it moved or linked something.
# boomdrivedev/sluice's nix/modules/activate.sh at 7ce112b4, for timber's
# path: timber has nothing to restart.
set -eu

usage() {
  echo "usage: activate.sh link <file>" >&2
  exit 2
}

link() {
  file=$1
  folder=$HOME/.config/timber
  target=$folder/config.toml
  aside=$target.before-nix

  if [ -L "$target" ] && [ "$(readlink "$target")" = "$file" ]; then
    return
  fi
  if [ -e "$target" ] && [ ! -L "$target" ]; then
    if [ -e "$aside" ] || [ -L "$aside" ]; then
      echo "timber: $target is no link and $aside is there already: left both, linked nothing; move one aside to link $file" >&2
      exit 1
    fi
    mv "$target" "$aside"
    echo "timber: moved $target to $aside"
  fi
  mkdir -p "$folder"
  ln -sfn "$file" "$target"
  echo "timber: linked $target to $file"
}

case ${1:-} in
link)
  [ $# -eq 2 ] || usage
  link "$2"
  ;;
*) usage ;;
esac
