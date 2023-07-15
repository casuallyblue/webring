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

      hostname = mkOption {
        type = types.str;
        default = "casuallyblue.dev";
        description = "The hostname to proxy to the server";
      };

      acmeEmail = mkOption {
        type = types.str;
        default = "amylarane@gmail.com";
        description = "The email to send certbot renewals to";
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

    services.nginx.enable = true;
    services.nginx.virtualHosts."casuallyblue.dev" = {
      forceSSL = true;
      enableACME = true;
      locations."/" = {
        proxyPass = "http://127.0.0.1:${builtins.toString cfg.port}";
        proxyWebsockets = true;
      };
    };
    
    security.acme.certs."${cfg.hostname}".email = cfg.acmeEmail;

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
