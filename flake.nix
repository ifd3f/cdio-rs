{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs, ... }:
    let pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
      devShells.x86_64-linux.default = with pkgs;
        mkShell {
          RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";
          LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
          buildInputs = [
            rustfmt
            rustc
            rust.packages.stable.rustPlatform.rustLibSrc
            cargo
            clang
            libcdio
          ];
        };
    };
}
