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

      vulkan-loader
   ]);

   shellHook = ''
      exec ${pkgs.fish}/bin/fish
   '';
}
