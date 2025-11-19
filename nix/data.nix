{
  stdenvNoCC,
  pasta,
  pesto,
  salt,
}:

stdenvNoCC.mkDerivation {
  name = "nixdle-data";
  src = ./.;

  buildPhase = ''
    runHook preBuild

    echo "Building nixdle data..."
    mkdir -p $out
    echo "Using pasta from: ${pasta}"
    cp ${salt}/builtins.types.json $out/builtins.types.json
    echo "Generating data.json..."
    ${pesto}/bin/pesto --pos-file ${pasta} --format json --language ${salt}/language.json $out/data.json
    echo "Data generation complete."

    runHook postBuild
  '';
}
