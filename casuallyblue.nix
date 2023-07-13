self: {config, lib, options, ...}: 
with lib;
let cfg = config.casuallyblue.services.site; in {
  options = {
    casuallyblue.services.site = {
      enable = mkEnableOption "casuallyblue.dev server";

      port = mkOption {
        type = types.port;
        default = 33241;
        example = 9001;
        description = "The http port to listen on";
      };      
    };
  };

  config = mkIf cfg.enable {
    users.users.cbsite = {
      createHome = true;
      description = "site";
      isSystemUser = true;
      group = "users";
      home = "/home/casuallyblue-site";
    };

    systemd.services."casuallyblue-dev-site" = {
      wantedBy = ["multi-user.target"];

      serviceConfig = {
        User = "cbsite";
        Group = "users";
        Restart=  "on-failure";
        WorkingDirectory = "/var/www/casuallyblue.dev";
        RestartSec = "30s";
        Type = "simple";
      };

      script = let site = self.packages.x86_64-linux.default;
      in ''
        exec ${site}/bin/site ${builtins.toString cfg.port}
      '';
    };
  };
}
