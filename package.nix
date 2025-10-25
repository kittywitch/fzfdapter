{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage (_finalAttrs: {
  pname = "fzfdapter";
  version = "0.1.0";

  src = ./.;

  cargoHash = "sha256-aaLgttzAlHJciCDn9vQ2bHPoNc6lcXQa4GIJQPvUgyw=";

  meta = {
    description = "fzfdapter, a fuzzel/wofi/rofi... thing for your terminal";
    homepage = "https://github.com/kittywitch/fzfdapter";
    license = lib.licenses.gpl3;
    maintainers = [];
  };
})
