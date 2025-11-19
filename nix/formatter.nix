{
  treefmt,
  nixfmt,
  rustfmt,
  shfmt,
}:

treefmt.withConfig {
  runtimeInputs = [
    nixfmt
    rustfmt
    shfmt
  ];

  settings = {
    on-unmatched = "info";
    tree-root-file = "flake.nix";

    formatter = {
      nixfmt = {
        command = "nixfmt";
        includes = [ "*.nix" ];
      };
      rustfmt = {
        command = "rustfmt";
        options = [
          "--edition"
          "2024"
        ];
        includes = [ "*.rs" ];
      };
      shfmt = {
        command = "shfmt";
        options = [ "-w" ];
        includes = [ "*.sh" ];
      };
    };
  };
}
