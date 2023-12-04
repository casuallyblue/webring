self: { config, lib, options, ... }:
with lib;
let cfg = config.casuallyblue.services.webring; in
{
  options = {
    casuallyblue.services.webring = {
      enable = mkEnableOption "webring.casuallyblue.dev service";

      port = mkOption {
        type = types.port;
        default = 33242;
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
    users.users.webring = {
      createHome = true;
      description = "site";
      isSystemUser = true;
      group = "users";
      home = "/var/lib/webring";
    };

    services.nginx.virtualHosts."${cfg.hostname}" = {
      forceSSL = true;
      enableACME = true;
      locations."/" = {
        proxyPass = "http://127.0.0.1:${builtins.toString cfg.port}";
        proxyWebsockets = true;
      };
    };

    security.acme.certs."${cfg.hostname}".email = cfg.acmeEmail;

    systemd.services."casuallyblue-webring" = {
      wantedBy = [ "multi-user.target" ];

      serviceConfig = {
        User = "webring";
        Group = "users";
        Restart = "on-failure";
        WorkingDirectory = config.users.users.webring.home;
        RestartSec = "30s";
        Type = "simple";
      };

      script =
        let
          package = self.packages.x86_64-linux.default;
          static-files = self.packages.x86_64-linux.static-files;
        in
        ''
          cd ${static-files}
          exec ${package}/bin/site ${builtins.toString cfg.port}
        '';
    };
  };
}
