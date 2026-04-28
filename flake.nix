{
   inputs = {
      nixpkgs.url = "nixpkgs/nixos-25.11";
      flake-parts.url = "github:hercules-ci/flake-parts";
   };

   outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
         "x86_64-linux"
         "aarch64-linux"
         "x86_64-darwin"
         "aarch64-darwin"
      ];

      perSystem = { pkgs, self', ... }: {
         packages.default = import ./package.nix { inherit pkgs self'; };
         devShells.default = import ./shell.nix { inherit pkgs self';  };
      };
   };
}
