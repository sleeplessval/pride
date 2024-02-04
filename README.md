
# Pride! for the terminal

A Rust utility to display pride flags in the terminal.

A list of currently implemented flags is available on the [project wiki](https://git.vwolfe.io/valerie/pride/wiki/Flags).

## Dependencies

Some Complex renderers utilize [Powerline's](https://github.com/ryanoasis/powerline-extra-symbols)
slant symbols, and therefore require use of a Powerline font, such as [Fira Code](https://github.com/tonsky/FiraCode).

## Installation

### From Binary

Copy the compiled binary from the [releases page](https://git.vwolfe.io/valerie/pride/releases)
to a directory in `$PATH`, such as `/usr/bin/`.

### From Source

Compile using cargo with the command `cargo build --release` and copy the file
from `target/release/` to a directory in `$PATH`, such as `/usr/bin/`.

### Arch Linux (AUR)

Install the package from the [`pride` AUR Package](https://aur.archlinux.org/packages/pride),
either using an AUR package manager, or by cloning the [AUR Repository](https://aur.archlinux.org/pride.git)
and running the command `makepkg --install`.

## Libraries

- [pico-args](https://crates.io/crates/pico-args) — argument parsing
- [termion](https://crates.io/crates/termion) — ANSI formatting

