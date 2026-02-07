# find-up

Find the nearest ancestor directory that contains a file or directory.

Starting from the current directory, `find-up` walks up the directory tree and prints the absolute path to the first match it finds. If no match is found, it exits with code 1.

## Installation

```sh
cargo install find-up
```

## Usage

```
find-up [--std] <name> [name ...]
```

### Search for specific files

```sh
# Find the nearest Cargo.toml
find-up Cargo.toml

# Search for multiple names at once (first match at the closest level wins)
find-up package.json Cargo.toml go.mod
```

### Use the built-in standard set

The `--std` flag searches for a curated set of common project-root markers including VCS directories and language/build-tool files:

```sh
find-up --std
```

This checks for `.git`, `.jj`, `package.json`, `Cargo.toml`, `go.mod`, `pyproject.toml`, and many more.

## Exit codes

| Code | Meaning |
|------|---------|
| 0    | Match found (path printed to stdout) |
| 1    | No match found |
| 2    | Usage error or failed to get current directory |

## License

MIT
