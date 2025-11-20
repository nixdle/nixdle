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

    mkdir -p $out
    cp ${salt}/builtins.types.json $out/builtin_types.json
    ${pesto}/bin/pesto --pos-file ${pasta} --format json --language ${salt}/language.json $out/functions.json

    runHook postBuild
  '';
}
