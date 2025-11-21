{
  description = "fzfdapter; kinda like J4-dmenu-desktop but not";
  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs = {
        flake-compat.follows = "flake-compat";
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    flake-utils,
    nixpkgs,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      treefmtEval = inputs.treefmt-nix.lib.evalModule inputs.nixpkgs.legacyPackages.${system} ./treefmt.nix;
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.default = import ./shell.nix {inherit pkgs;};

      packages = let
        fzfdapter = pkgs.callPackage ./package.nix {};
      in {
        inherit fzfdapter;
        default = fzfdapter;
      };

      formatter = treefmtEval.config.build.wrapper;
      checks = let
        git-hooks = system:
          inputs.git-hooks.lib.${system}.run {
            src = ./.;
            hooks = {
              treefmt = {
                enable = true;
                packageOverrides = {treefmt = inputs.self.formatter.${system};};
              };
              flake-checker.enable = true;
              ripsecrets.enable = true;
            };
          };
      in {
        formatting = treefmtEval.config.build.check inputs.self;
        git-hooks = git-hooks system;
      };
    });
}
