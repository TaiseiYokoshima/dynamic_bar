{ pkgs, self' }:
pkgs.mkShell {
   buildInputs = with pkgs; [
      wayland
      wayland-protocols
      libxkbcommon
      libinput
   ];


   LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath ( with pkgs; [
      wayland
      wayland-protocols
      libxkbcommon
      libinput
   ]);

   shellHook = ''
      export XKB_CONFIG_ROOT=${pkgs.xkeyboard_config}/share/X11/xkb
      exec ${pkgs.fish}/bin/fish
   '';
}
