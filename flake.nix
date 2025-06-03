{
  description = "Build Shell with any dependency of the project";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem(system:
        let pkgs = import nixpkgs {
              inherit system;
              overlays = [rust-overlay.overlays.default];
            };
            toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
        in
        {
          devShell = pkgs.mkShell {
            packages = [
              toolchain
              pkgs.rust-analyzer-unwrapped

              # Necessary for the openssl-sys crate:
              pkgs.openssl
              pkgs.pkg-config

              # GB Dev tools
              pkgs.rgbds

              # Python
              pkgs.python312
              pkgs.python312Packages.requests
            ];

            RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";

            shellHook = ''
if emacsclient -e "(emacs-pid)" >/dev/null 2>&1; then
    # Emacs daemon is running
    if [ "$(emacsclient -e '(length (frame-list))' 2>/dev/null | tr -d '\"')" -le 1 ]; then
      # No GUI or TTY client connected (only the initial frame)
      echo "Killing Emacs daemon for updated environment..."
      emacsclient -e '(kill-emacs)' >/dev/null 2>&1
    else
      echo "Emacs clients are connected; skipping daemon kill."
    fi
  else
    echo "No emacs daemon running, nothing to kill"
  fi
'';
          };
        }
      );
}
