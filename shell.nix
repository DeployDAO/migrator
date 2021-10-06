{ pkgs }:
let
  anchor-parse-idls = pkgs.writeShellScriptBin "anchor-parse-idls"
    (builtins.readFile ./scripts/idl.sh);
in pkgs.mkShell {
  nativeBuiltInputs = (pkgs.lib.optionals pkgs.stdenv.isDarwin [
    pkgs.darwin.apple_sdk.frameworks.AppKit
    pkgs.darwin.apple_sdk.frameworks.IOKit
    pkgs.darwin.apple_sdk.frameworks.Foundation
  ]);
  buildInputs = with pkgs;
    (pkgs.lib.optionals pkgs.stdenv.isLinux ([
      # solana
      libudev
    ])) ++ [
      anchor-parse-idls
      rustup
      cargo-deps
      # cargo-watch
      gh

      # sdk
      nodejs
      yarn
      python3

      pkgconfig
      openssl
      jq
      gnused

      libiconv

      anchor-0_17_0
      spl-token-cli
    ] ++ (pkgs.lib.optionals pkgs.stdenv.isDarwin [
      pkgs.darwin.apple_sdk.frameworks.AppKit
      pkgs.darwin.apple_sdk.frameworks.IOKit
      pkgs.darwin.apple_sdk.frameworks.Foundation
    ]);
  shellHook = ''
    export PATH=$PATH:$HOME/.cargo/bin
  '';
}
