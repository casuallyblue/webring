ssh sierra@casuallyblue.dev "cd configuration && nix flake lock --update-input $@"
ssh sierra@casuallyblue.dev -t "sudo nixos-rebuild switch --flake ./configuration"