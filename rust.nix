{pkgs ? import <nixos-unstable> {}}:
pkgs.mkShell {
  shellHook = ''
    alias build="cargo build"
    alias run="cargo run"

    echo "You are now in a rust shell"
    echo "Currently in $(pwd)"
  '';

  nativeBuildInputs = with pkgs.buildPackages; [
    rustc
    cargo
    rust-analyzer

    # glibc.static
    pkg-config
    openssl
  ];
}
