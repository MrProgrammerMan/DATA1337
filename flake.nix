{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs @ { flake-parts, fenix, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" ];
    perSystem = { pkgs, system, ... }: 
    let
      f = with fenix.packages.${system}; combine [
        complete.toolchain
        targets.wasm32-unknown-unknown.latest.rust-std
      ];
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          f
          trunk
          leptosfmt
        ];
      };
    };
  };
}
