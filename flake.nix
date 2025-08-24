{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };

    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc

          pay-respects
          (vscode-with-extensions.override {
            #             vscode = vscodium;

            vscodeExtensions = with vscode-extensions; [
              vscode-extensions.rust-lang.rust-analyzer

              continue.continue
              fill-labs.dependi
              mhutchie.git-graph
              vscodevim.vim
            ];
          })
        ];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

        shellHook = ''
          SHELL=${pkgs.zsh}/bin/zsh
          code -w .

          exit
        '';
      };
      formatter.${system} = pkgs.nixfmt-tree;

    };

}
