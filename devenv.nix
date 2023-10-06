{ lib, pkgs, ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";

    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  pre-commit.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };

  packages = with pkgs; [
    openssl
  ];

  #packages = [ pkgs.git ];
  #env.GREET = "devenv";
  #scripts.hello.exec = "echo hello from $GREET";
  #enterShell = ''
  #  hello
  #  git --version
  #'';
}
