{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";

  outputs =
    { self, nixpkgs }:
    let
      eachSystem =
        f:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ] f;
    in
    {
      devShells = eachSystem (
        system: with nixpkgs.legacyPackages.${system}; {
          default = mkShell {
            packages = [
              cargo
              cargo-expand
              rustc
              rust-analyzer
              rustfmt
            ];
          };
        }
      );
      packages = eachSystem (system: rec {
        theme-manager =
          let
            cargo = (nixpkgs.lib.importTOML ./Cargo.toml).package;
          in
          nixpkgs.legacyPackages.${system}.rustPlatform.buildRustPackage {
            pname = cargo.name;
            version = cargo.version;

            src = ./.;

            cargoLock.lockFile = ./Cargo.lock;

            meta = {
              description = cargo.description;
              homepage = cargo.homepage;
              license = nixpkgs.lib.licenses.gpl3Only;
              maintainers = with nixpkgs.lib.maintainers; [ samuel-martineau ];
            };

            nativeBuildInputs = with nixpkgs.legacyPackages.${system}; [ installShellFiles ];

            postInstall = ''
              installShellCompletion --cmd ${cargo.name} ./completions.fish
            '';
          };
        default = theme-manager;
      });
    };
}
