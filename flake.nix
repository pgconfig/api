{
  description = "PostgreSQL Configuration Builder";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    gopkgs.url = "github:sagikazarmark/go-flake";
    gopkgs.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, gopkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        version = "3.1.4";

        commit = if (self ? rev) then self.rev else "dirty";
        ldFlags = [
          "-s"
          "-w"
          "-X github.com/pgconfig/api/pkg/version.Tag=${version}"
          "-X github.com/pgconfig/api/pkg/version.Commit=${commit}"
        ];
        pgconfig = pkgs.buildGoModule {
          pname = "pgconfig";
          version = version;
          src = ./.;
          vendorHash = "sha256-GrKMo/xDOllRJMxDKR6ufdCXqJdJJ1zlgupnH35MoS8=";
          subPackages = [ "./cmd/api" "./cmd/pgconfigctl" ];
          goDependencies = with pkgs; [ go-swag ];
          postInstall = ''
            mkdir -p $out/shared/
            cp -rv ./*.yml $out/shared/
          '';
          preBuild = ''
            echo "Executando swag init..."
            ${pkgs.go-swag}/bin/swag init --dir ./cmd/api --output ./cmd/api/docs

            echo "Rodando testes..."
            # go test ./.. 
          '';
          goBuildFlags =
            [ "-ldflags" "${nixpkgs.lib.concatStringsSep " " ldFlags}" ];
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [ go gopkgs.mga go-swag goreleaser ];
        };
        packages = {
          default = pgconfig;
          image = let
            port = "3000";
            docsFile = builtins.readFile ./pg-docs.yml;
          in pkgs.dockerTools.buildImage {
            name = "docker.io/pgconfig/api";
            tag = "latest";

            copyToRoot = pkgs.buildEnv {
              name = "image-root";
              pathsToLink = [ "/bin" ];
              paths = [ pkgs.cacert pgconfig ];
            };
            created = "now";

            config = {
              WorkingDir = "${pgconfig}";
              Entrypoint = [
                "./bin/api"
                "--port=${port}"
                "--docs-file=./shared/pg-docs.yml"
                "--rules-file=./shared/rules.yml"
              ];
              ExposedPorts = { "${port}/tcp" = { }; };
            };
          };

        };
      });
}
