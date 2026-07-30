{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "dwsh";

        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          rust-analyzer
          clippy
          rustfmt
          eslint
          prettier

          pkg-config
        ];

        buildInputs = with pkgs; [
          pango
          gtk4
          gtk4-layer-shell
          librsvg
          dbus
        ];
      };
    };
}
