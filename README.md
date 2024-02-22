
# Pride! for the terminal

A Rust utility to display pride flags in the terminal.

A list of currently implemented flags is available on the [project wiki](https://git.vwolfe.io/valerie/pride/wiki/Flags).

## Dependencies

Some Complex renderers utilize [Powerline's](https://github.com/ryanoasis/powerline-extra-symbols)
slant symbols, and therefore require use of a Powerline font, such as [Fira Code](https://github.com/tonsky/FiraCode).

## Installation

### Manual Install

<details>
<summary>From Binary</summary>
Copy the compiled binary from the <a href="https://git.vwolfe.io/valerie/pride/releases/">releases page</a>
to a directory in <code>$PATH</code>, such as <code>/usr/bin/</code>.
</details>

<details>
<summary>Compile from Source</summary>
Compile using cargo with the command <code>cargo build --release</code> and
copy the file from <code>target/release/</code> to a directory in
<code>$PATH</code>, such as <code>/usr/bin/</code>.
</details>

<details>
<summary>makepkg (AUR)</summary>
Clone the <a href="https://aur.archlinux.org/pride.git">AUR Repository</a> and
run the command <code>makepkg --install</code>
</details>

### Package Managers

<details>
<summary>Arch Linux (AUR): <code>pride</code></summary>
Install the package from the <a href="https://aur.archlinux.org/packages/pride"><code>pride</code> AUR Package</a>
using an AUR package manager such as <a href="https://github.com/Morganamilo/paru"><code>paru</code></a>
</details>

<details>
<summary>Cargo: <code>pride-term</code></summary>
Install the package using Cargo with the command <code>cargo install pride-term</code>.
</details>

## Libraries

- [pico-args](https://crates.io/crates/pico-args) — argument parsing
- [termion](https://crates.io/crates/termion) — ANSI formatting

