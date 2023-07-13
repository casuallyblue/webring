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
      group = "within";
      home = "/var/www/casuallyblue.dev";
    };

    systemd.services."casuallyblue-dev-site" = {
      wantedBy = ["multi-user.target"];

      serviceConfig = {
        User = "cbsite";
        Group = "within";
        Restart=  "on-failure";
        WorkingDirectory = "/var/www/casuallyblue.dev";
        RestartSec = "30s";
        Type = "notify";
      };

      script = let site = self.packages.${system}.default;
      in ''
        cd ${site}
        exec ${site}/bin/site
      '';
    };
  };
}
