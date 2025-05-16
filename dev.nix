{pkgs ? import <nixos-unstable> {}}:
pkgs.mkShell {
  shellHook = ''
    alias build="cargo build"
    alias run="cargo run"

    echo "You are now in a rust shell"
    echo "Currently in $(pwd)"
<<<<<<< HEAD
=======
    export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS;
>>>>>>> 95c5656 (added tauri)
  '';

  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    cargo
    cargo-tauri
    nodejs
<<<<<<< HEAD
  ];

  buildInputs = with pkgs;[
=======
    rustc
    pnpm
  ];

  buildInputs = with pkgs;[
    gtk3
    gsettings-desktop-schemas
>>>>>>> 95c5656 (added tauri)
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
