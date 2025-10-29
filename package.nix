{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage (_finalAttrs: {
  pname = "fzfdapter";
  version = "0.1.0";

  src = ./.;

  cargoHash = "sha256-gwaH/Q9VN1i3JLruj6aRBhInWy+qHV+g32wSKY++msw=";

  meta = {
    mainProgram = "fzfdapter";
    description = "fzfdapter, a fuzzel/wofi/rofi... thing for your terminal";
    homepage = "https://github.com/kittywitch/fzfdapter";
    license = lib.licenses.gpl3;
    maintainers = [];
  };
})
