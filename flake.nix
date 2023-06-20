{
  description = "NVIDIA NVAPI bindings for Rust";
  inputs = {
    flakelib.url = "github:flakelib/fl";
    nixpkgs = { };
    rust = {
      url = "github:arcnmx/nixexprs-rust";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, flakelib, nixpkgs, rust, ... }@inputs: let
    nixlib = nixpkgs.lib;
  in flakelib {
    inherit inputs;
    systems = [ "x86_64-linux" "aarch64-linux" ];
    devShells = {
      plain = {
        mkShell, hostPlatform
      , enableRust ? true, cargo
      , rustTools ? [ ]
      , generate
      }: mkShell {
        inherit rustTools;
        nativeBuildInputs =
          nixlib.optional enableRust cargo
          ++ [ generate ];
      };
      stable = { rust'stable, outputs'devShells'plain }: outputs'devShells'plain.override {
        inherit (rust'stable) mkShell;
        enableRust = false;
      };
      dev = { rust'unstable, rust-w64-overlay, outputs'devShells'plain }: let
        channel = rust'unstable.override {
          channelOverlays = [ rust-w64-overlay ];
        };
      in outputs'devShells'plain.override {
        inherit (channel) mkShell;
        enableRust = false;
        rustTools = [ "rust-analyzer" ];
      };
      default = { outputs'devShells }: outputs'devShells.plain;
    };
    packages = {
    };
    legacyPackages = { callPackageSet }: callPackageSet {
      source = { rust'builders }: rust'builders.wrapSource self.lib.crate.src;

      rust-w64 = { pkgsCross'mingwW64 }: import inputs.rust { inherit (pkgsCross'mingwW64) pkgs; };
      rust-w64-overlay = { rust-w64 }: let
        target = rust-w64.lib.rustTargetEnvironment {
          inherit (rust-w64) pkgs;
          rustcFlags = [ "-L native=${rust-w64.pkgs.windows.pthreads}/lib" ];
        };
      in cself: csuper: {
        sysroot-std = csuper.sysroot-std ++ [ cself.manifest.targets.${target.triple}.rust-std ];
        cargo-cc = csuper.cargo-cc // cself.context.rlib.cargoEnv {
          inherit target;
        };
        rustc-cc = csuper.rustc-cc // cself.context.rlib.rustcCcEnv {
          inherit target;
        };
      };

      generate = { rust'builders, outputHashes }: rust'builders.generateFiles {
        paths = {
          "lock.nix" = outputHashes;
        };
      };
      outputHashes = { rust'builders }: rust'builders.cargoOutputHashes {
        inherit (self.lib) crate;
      };
    } { };
    checks = {
      versions = { rust'builders, source }: rust'builders.check-contents {
        src = source;
        patterns = [
          { path = "src/lib.rs"; docs'rs = {
            inherit (self.lib.crate) name version;
          }; }
          { path = "sys/src/lib.rs"; docs'rs = {
            inherit (self.lib.crate.members.sys) name version;
          }; }
          { path = "hi/src/lib.rs"; docs'rs = {
            inherit (self.lib.crate.members.hi) name version;
          }; }
        ];
      };
      test = { rustPlatform, outputs'devShells'plain, source }: rustPlatform.buildRustPackage {
        pname = self.lib.crate.package.name;
        inherit (self.lib.crate) cargoLock version;
        inherit (outputs'devShells'plain.override { enableRust = false; }) buildInputs nativeBuildInputs;
        src = source;
        cargoBuildNoDefaultFeatures = true;
        cargoTestFlags = [ "--workspace" "--all-targets" ];
        buildType = "debug";
        meta.name = "cargo test";
      };
      windows = { outputs'checks'test, rust-w64 }: rust-w64.latest.rustPlatform.buildRustPackage {
        inherit (outputs'checks'test) pname version src buildType cargoBuildNoDefaultFeatures cargoTestFlags;
        inherit (self.lib.crate) cargoLock;

        cargoBuildFlags = outputs'checks'test.cargoTestFlags;
        meta.name = "cargo build --target ${rust-w64.latest.hostTarget.triple}";
      };
    };
    lib = {
      crate = rust.lib.importCargo {
        inherit self;
        path = ./Cargo.toml;
        inherit (import ./lock.nix) outputHashes;
      };
      inherit (self.lib.crate.package) version;
    };
    config = rec {
      name = "nvapi-rs";
      packages.namespace = [ name ];
    };
  };
}
