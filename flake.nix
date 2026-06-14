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
        texlive-toolchain = pkgs.texliveSmall.withPackages (
          ps: with ps; [
            latexmk
            todonotes
            parskip
            cleveref
            xargs
            silence
            csquotes
            listings
            xcolor
            babel
            hyperref
            amsmath
            geometry
            titling
            xstring
            enumitem
            setspace
            fancyhdr
            acronym
            bigfoot
            uml
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
              texlive-toolchain
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
