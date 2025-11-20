{
  lib,
  rustPlatform,
  data,
}:

let
  p = (lib.importTOML ../../Cargo.toml).workspace.package;
in
rustPlatform.buildRustPackage {
  pname = "nixdle-server";
  inherit (p) version;

  src = ../..;

  cargoLock.lockFile = ../../Cargo.lock;
  cargoBuildFlags = "--package nixdle-server --bin nixdle-server";

  env.NIXDLE_DATA_DIR = data;

  meta = {
    inherit (p) description;
    homepage = p.repository;
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ adamperkowski ];
    mainProgram = "nixdle-server";
  };
}
