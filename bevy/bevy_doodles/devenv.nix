{ pkgs, ... }:

let
  libraries = with pkgs; [
    udev
    alsa-lib
    vulkan-loader
    libx11
    libxcursor
    libxi
    libxrandr
    libxkbcommon
    wayland
  ];
in
{
  cachix.enable = false;

  languages.rust = {
    enable = true;
    channel = "stable";
  };

  packages = with pkgs; [
    pkg-config
    cmake
  ] ++ libraries;

  env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libraries;

  enterShell = ''
    echo "Bevy 0.18 development environment"
    echo "Run 'cargo run' to start your project"
  '';
}
