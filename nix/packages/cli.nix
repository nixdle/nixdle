{
  lib,
  rustPlatform,
}:

let
  p = (lib.importTOML ../../Cargo.toml).workspace.package;
in
rustPlatform.buildRustPackage {
  pname = "nixdle";
  inherit (p) version;

  src = ../..;

  cargoLock.lockFile = ../../Cargo.lock;
  cargoBuildFlags = "--package nixdle-cli --bin nixdle";

  doCheck = false;

  meta = {
    inherit (p) description;
    homepage = p.repository;
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ adamperkowski ];
    mainProgram = "nixdle";
  };
}
