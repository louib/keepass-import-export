# keepass-import-export
[![Build Status](https://github.com/louib/keepass-import-export/actions/workflows/merge.yml/badge.svg?branch=main)](https://github.com/louib/keepass-import-export/actions/workflows/merge.yml)
[![dependency status](https://deps.rs/repo/github/louib/keepass-import-export/status.svg)](https://deps.rs/repo/github/louib/keepass-import-export)
[![License file](https://img.shields.io/github/license/louib/keepass-import-export)](https://github.com/louib/keepass-import-export/blob/main/LICENSE)

CLI tools to export to and import from KDBX (keepass) databases

`keepass-import-export` is based on the [`keepass-rs` library](https://github.com/sseemayer/keepass-rs).

## Installing

### With Nix
Assuming that you have enabled both the `flakes` and `nix-command` experimental features:
```
nix profile install github:louib/keepass-import-export
```

### With Cargo
```
cargo install --path .
```
