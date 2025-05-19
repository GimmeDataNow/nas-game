{pkgs ? import <nixos-unstable> {}}:
pkgs.mkShell {
  shellHook = ''
    alias build="cargo build"
    alias run="cargo run"
    alias trun="pnpm run tauri dev"

    echo "You are now in a rust shell"
    echo "Currently in $(pwd)"

    export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS;
  '';

  nativeBuildInputs = with pkgs; [
    pkg-config
    gobject-introspection
    cargo
    cargo-tauri
    nodejs
  ];

  buildInputs = with pkgs; [
    typescript-language-server
    rustc
    pnpm
    gtk3
    gsettings-desktop-schemas
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
