{
  description = "golem-cli";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }:
  let
    pkgs = import nixpkgs { system = "x86_64-linux"; };
  in {
    packages.x86_64-linux.default = with pkgs; rustPlatform.buildRustPackage {
      pname = "golem-cli";
      version = "0.0.98";
      nativeBuildInputs = [ pkg-config ];
      buildInputs = [ pkg-config protobuf openssl ];
      LD_LIBRARY_PATH = lib.makeLibraryPath [ openssl ];
      PROTOC = "${pkgs.protobuf}/bin/protoc";
      # OPENSSL_DIR = pkgs.openssl.dev;
      # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      cargoLock = {
        lockFile = ./Cargo.lock;
        outputHashes = {
          "cranelift-bforest-0.104.0" = "sha256-veZc4s+OitjBv4ohzzgFdAxLm/J/B5NVy+RXU0hgfwQ=";
          "libtest-mimic-0.7.0" = "sha256-xUAyZbti96ky6TFtUjyT6Jx1g0N1gkDPjCMcto5SzxE=";
        };
      };
      src = pkgs.lib.cleanSource ./.;
    };
  };

}
