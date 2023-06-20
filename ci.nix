{ pkgs, lib, ... }: with pkgs; with lib; let
  inherit (import ./. { inherit pkgs; }) checks;
in {
  config = {
    name = "nvapi";
    ci = {
      version = "v0.6";
      gh-actions.enable = true;
    };
    cache.cachix.arc.enable = true;
    channels = {
      nixpkgs = "23.05";
    };
    tasks = {
      build.inputs = singleton checks.test;
    };
    jobs = {
      nixos = {
        tasks = {
          windows.inputs = singleton checks.windows;
        };
      };
    };
  };
}
