{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [rust-overlay.overlays.default];
    };
    toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        toolchain

        pkgs.libxkbcommon pkgs.libGL pkgs.wayland pkgs.xorg.libXcursor
        pkgs.xorg.libXrandr pkgs.xorg.libXi pkgs.xorg.libX11
      ];

      LD_LIBRARY_PATH = builtins.concatStringsSep ":" [
        "${pkgs.xorg.libX11}/lib"
        "${pkgs.xorg.libXi}/lib"
        "${pkgs.libGL}/lib"
      ];
    };
  };
}
