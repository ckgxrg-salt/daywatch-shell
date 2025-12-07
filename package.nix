{
  rustPlatform,
  lib,
  pkg-config,
  pango,
  librsvg,
  gtk4,
  gtk4-layer-shell,
}:
rustPlatform.buildRustPackage {
  pname = "dwsh";
  version = "0.0.1";

  src = ./.;
  cargoHash = "sha256-gUu3PnW2ICpXof6R6xB57eThO9OwcQH7O23t0cEoEbs=";

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    pango
    librsvg
    gtk4
    gtk4-layer-shell
  ];

  meta = {
    description = "A desktop shell for my laptop";
    homepage = "https://codeberg.org/ckgxrg/dwsh";
    license = lib.licenses.bsd2;
    maintainers = with lib.maintainers; [ ckgxrg ];
  };
}
