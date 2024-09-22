# Wots - Basically GNU Stow, but much simpler and lightweight

## Installation

```sh
$ git clone https://github.com/squarejellyfish/wots.git
$ cd wots
$ make install
```

This installation will generate some global files, build the binary, and install it (defaults to `/usr/local/bin/`).

To only build the binary and install manually, just do:

```sh
$ make manual
```

And copy or link `/target/release/wots` to the designated directory.

## Usage

```
wots [OPTIONS] <FILE_NAME>

Arguments:
  <FILE_NAME>  Path to the file to symlink, set to "." will link all the files in the current directory. Respects .gitignore

Options:
  -t, --target-path <DIR>  Target directory, default is home directory [default: /Users/xuchengru]
  -d, --delete             Delete (unstow) the package from the target directory if this option is enabled
  -f, --force              Force the link(s) if they already exist
  -h, --help               Print help
  -V, --version            Print version
```

## Ignore files

Wots will check any file named `.wots-ignore` in the current working directory. If it does not exist, Wots will check the global ignore file `~/.wots-global-ignore` (this is generated automatically if `make install` or `make manual` is run).

The Wots ignore file uses regex matching to determine if a file should be ignored. Here's an example of a Wots ignore file:

```
dontlinkme.sh
dont.*.sh
```

`dontlinkme.sh` will actually match `dontlinkmeOsh`, where `O` can be any character. This will be fixed in the future.

## TODO

1. Fix regex matching: `.` in the file extension is recognized as a regex symbol now.
