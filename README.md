<div align="center">
  <h1>nixdle</h1>
  <p>wordle but it's nix functions</p>
</div>

## about
[about]: #about

the "official" nixdle instance is hosted at [adamperkowski.dev/api/nixdle]

as you can probably see, this README and docs are almost non-existent and since the project is still a baby, pls give us some time to improve them :D

## how to play
[how to play]: #how-to-play

to play nixdle, simply run

```sh
cachix use nixdle # optional, but recommended
nix run github:nixdle/nixdle/stable
```

## contributing
[contributing]: #contributing

if you're looking to contribute to nixdle, might be wosth to check out the [todo list] first!

also pls see the [contributing guidelines]

## thanks to <3
[thanks to]: #thanks

- [noogle] for dataset generation tools that made this possible (pls stop using flake-parts tho)

[adamperkowski.dev/api/nixdle]: https://adamperkowski.dev/api/nixdle
[todo list]: ./TODO
[contributing guidelines]: CONTRIBUTING.md
[noogle]: https://github.com/nix-community/noogle
