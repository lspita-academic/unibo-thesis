{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        texLive-toolchain = pkgs.texliveMedium.withPackages (
          ps: with ps; [
            latexmk
            todonotes
            cleveref
            xargs
            listings
            xcolor
            babel
            hyperref
            amsmath
            geometry
            titling
            enumitem
            setspace
            fancyhdr
            acronym
            bigfoot
          ]
        );
      in
      {
        devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              # nix
              nixd
              nil
              nixfmt
              # yaml
              yaml-language-server
              # latex
              texLive-toolchain
              texlab
            ];
            shellHook = ''
              if [ -f .env ]; then
                set -a
                source .env
                set +a
              fi
            '';
          };
      }
    );
}
