{
  description = "Ratatui Music Player Rust";
  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  outputs = { nixpkgs, ... }: {
    devShells.x86_64-linux.default = 
      let pkgs = nixpkgs.legacyPackages.x86_64-linux;
      in pkgs.mkShell {
        buildInputs = with pkgs; [ alsa-lib pkg-config ];
      };
  };
}
