# mity-rs

[https://github.com/KCCG/mity](https://github.com/KCCG/mity) but in rust :)

## Important!

Work in progress at the moment! Use at your own risk!

## Install
```bash
cargo install --git https://github.com/trentzz/mity-rs mity-rs
```

Prerequisites:
- `freebayes`
- `tabix`

## Usage
```bash
$ mity-rs -h
Mity RS: Mitochondrial variant analysis toolkit in rust

Usage: mity-rs <COMMAND>

Commands:
  call       Call mitochondrial variants
  normalise  Normalise & filter mitochondrial variants
  report     Generate mity report
  merge      Merge mity and nuclear VCF files
  runall     Run analysis on BAM/CRAM files
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```