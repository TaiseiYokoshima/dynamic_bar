{ pkgs, self' }:
pkgs.mkShell {
   buildInputs = with pkgs; [
      wayland
      wayland-protocols
      libxkbcommon
      # cargo
      # rustup
   ];


   LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath ( with pkgs; [
      wayland
      libxkbcommon
   ]);

   # export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}/lib:$LD_LIBRARY_PATH

   shellHook = ''
      exec ${pkgs.fish}/bin/fish
   '';
}
