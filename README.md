# check-ignore

A simple command line tool for checking if files are ignored by patterns in a .gitignore file.

### Install
```shell
cargo install check-ignore
```

### Usage
```rust
check-ignore 1.0.0
Exits with non-zero code if files provided don't match at least one pattern.
Outputs results of the form `pattern => file`.

USAGE:
    check-ignore [FLAGS] [OPTIONS] [FILES]...

FLAGS:
    -d, --debug        Activate debug mode
    -g, --global       Use global gitignore
    -h, --help         Prints help information
    -V, --version      Prints version information
    -v, --verbose      Verbose mode (-v, -vv, -vvv, etc.)
    -w, --whitelist    Also print whitelisted files

OPTIONS:
    -i, --ignore-file <ignore-file>    .gitignore file [default: .gitignore]
        --no-color <no-color>          Disable colorful output [env: NO_COLOR=]
    -r, --root <root>                  Root for checking file

ARGS:
    <FILES>...    Files to check
```