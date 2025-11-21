{
  callPackage,
  mkShell,
  clippy,
  rustfmt,
  rust-analyzer,
  shellcheck,
  cachix,
}:

let
  mainPkg = callPackage ./packages/cli.nix { };
  packages = [
    clippy
    rustfmt
    rust-analyzer
    shellcheck
    cachix
  ]
  ++ mainPkg.nativeBuildInputs;
in
mkShell {
  nativeBuildInputs = packages;
  shellHook = ''
    echo -ne "-----------------------------------\n "
    echo -n "${toString (map (pkg: "• ${pkg.name}\n") packages)}"
    echo "-----------------------------------"
  '';
}
