{
  description = "Build Shell with any dependency of the project";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem(system:
        let pkgs = import nixpkgs {
              inherit system;
              overlays = [(import rust-overlay)];
            };
            toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
        in
        {
          devShell = pkgs.mkShell {

            nativeBuildInputs = with pkgs; [
              toolchain
            ];

            buildInputs = with pkgs; [
              openssl
              pkg-config
              rust-analyzer

              # GB Dev tools
              pkgs.rgbds

              # Python
              pkgs.python312
              pkgs.python312Packages.requests

              # SDL3
              pkgs.sdl3
            ];

            shellHook = ''
              _TRW="$HOME/.rust-rover/toolchain"
              _VER="$(${toolchain}/bin/rustc -V | awk '{print $2}')"

              # Rebuild when the Rust version changes
              if [ ! -f "$_TRW/.version" ] || \
                 [ "$(cat "$_TRW/.version" 2>/dev/null)" != "$_VER" ]; then
                echo "Setting up RustRover toolchain ($_VER)..."
                rm -rf "$_TRW"
                mkdir -p "$_TRW/lib/rustlib/src/rust"

                # bin: symlink — executables are run, not written
                ln -sfn ${toolchain}/bin "$_TRW/bin"

                # lib: symlink everything except rustlib (needs partial override)
                for _f in ${toolchain}/lib/*; do
                  _n="$(basename "$_f")"
                  [ "$_n" = rustlib ] || ln -sfn "$_f" "$_TRW/lib/$_n"
                done

                # lib/rustlib: symlink everything except src (needs partial override)
                for _f in ${toolchain}/lib/rustlib/*; do
                  _n="$(basename "$_f")"
                  [ "$_n" = src ] || ln -sfn "$_f" "$_TRW/lib/rustlib/$_n"
                done

                # lib/rustlib/src/rust/library: writable COPY
                # Java's Files.copy preserves source permissions — if the source
                # is read-only (Nix store), the destination is too. A real copy
                # with u+w lets RustRover write into its stdlib-local-copy cache.
                cp -rL ${toolchain}/lib/rustlib/src/rust/library \
                       "$_TRW/lib/rustlib/src/rust/"
                chmod -R u+w "$_TRW/lib/rustlib/src/rust/library"

                echo "$_VER" > "$_TRW/.version"
              fi

              # Fix any existing stdlib-local-copy entries that already got bad
              # permissions (e.g. from a previous attempt with the old symlink setup)
              for _vdir in "$HOME/.cache/JetBrains"/RustRover*/intellij-rust/stdlib-local-copy/"$_VER"-*/; do
                [ -d "$_vdir" ] || continue
                [ ! -w "$_vdir/src" ] || continue
                echo "Fixing RustRover stdlib cache at $_vdir..."
                rm -rf "$_vdir/src"
                mkdir -p "$_vdir/src"
                cp -rL ${toolchain}/lib/rustlib/src/rust/library/. "$_vdir/src/"
                chmod -R u+w "$_vdir/src"
              done

              export RUST_SRC_PATH="$_TRW/lib/rustlib/src/rust/library"
            '';
          };
        }
      );
}
