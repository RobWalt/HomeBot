# shell.nix

{ pkgs ? import <nixos> { } }:
with pkgs; mkShell {
  nativeBuildInputs =
    [
      pkgconfig
      clang #
      lld #
    ];
  buildInputs =
    [
      openssl
    ];
  shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
    openssl
  ]}"'';
}
