# contributing guidelines

thanks for considering contributing to nixdle <3
really means a lot

## contents
[contents]: #contents

- [environment](#environment)
- [formatting](#formatting)
- [building and testing](#building-and-testing)
- [commit conventions](#commit-conventions)
- [license](#license)

## environment
[evnironment]: #environment

the project uses [nix] for its development environment and build system.
to get started, [install nix] and run:

```sh
nix develop
```

this will drop you into a shell with all the necessary tools and dependencies installed.
pretty cool huh.

## formatting
[formatting]: #formatting

we use [treefmt][treefmt] ([configured in nix][treefmt config]) to maintain consistent code formatting across the project.
to automatically format all files, run:

```sh
nix fmt
```

## building and testing
[building and testing]: #building-and-testing

you can use either [nix] or [cargo] to build the main `nixdle` binary and [cargo] to run tests on the library.
with nix, run:

```sh
nix build .#nixdle # for the CLI binary
nix build .#server # for the example server binary
nix build .#data   # for the dataset
```

with cargo, run:

```sh
cargo build --bin nixdle                         # for the CLI binary
cargo build --bin nixdle-server -p nixdle-server # for the example server binary
```

to run the test suite, use:

```sh
cargo test --lib
```

## commit conventions
[commit conventions]: #commit-conventions

use the [`check-commit-message` script] to validate your commit messages before pushing.

the script checks the following rules:
- commit messages must have a scope and a message, separated by `: `
- the scope must be one of the following:
  - `cli`
  - `lib`
  - `server`
  - `nix`
  - `ci`
  - `docs`
  - `chore`
- the message must start and end with a lowercase letter
- first line (the header) must not exceed 50 characters
- second line (if present) must be empty
- subsequent lines (if present) must not exceed 72 characters
- the header can optionally contain multiple messages separated by `; `

examples of valid commit messages:
- ``lib: init sqlite support``
- ``cli: add help command; docs: update readme``
- ``docs: refact contributing guidelines for clarity``

## license
[license]: #license

nixdle is licensed under the mit license.
when contributing, you agree that your contributions will be licensed under its terms (see the [license file]).

[nix]: https://nix.dev
[install nix]: https://nix.dev/install-nix
[treefmt]: https://treefmt.com
[treefmt config]: ./nix/formatter.nix
[cargo]: https://doc.rust-lang.org/cargo
[`check-commit-message` script]: ./scripts/check-commit-message.sh
[license file]: ./LICENSE
