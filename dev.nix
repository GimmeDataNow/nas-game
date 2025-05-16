{pkgs ? import <nixos-unstable> {}}:
pkgs.mkShell {
  shellHook = ''
    alias build="cargo build"
    alias run="cargo run"

    echo "You are now in a rust shell"
    echo "Currently in $(pwd)"
  '';

  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    cargo
    cargo-tauri
    nodejs
  ];

  buildInputs = with pkgs;[
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl
  ];
}
