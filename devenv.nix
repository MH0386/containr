{
  pkgs,
  lib,
  config,
  ...
}:

{
  files = {
    ".yamllint.yaml".yaml = {
      extends = "default";
      rules = {
        document-start = "disable";
        truthy = "disable";
        comments = "disable";
        line-length.max = 120;
      };
    };
  };

  packages = [
    pkgs.dioxus-cli

    # Linux desktop dependencies for Dioxus
    pkgs.webkitgtk_4_1
    pkgs.xdotool
    pkgs.openssl

    # GTK development libraries - required for building GTK applications
    # pkgs.pkg-config
    # pkgs.gtk3-x11
    # pkgs.cairo
    # pkgs.pango
    # pkgs.atk
    # pkgs.gdk-pixbuf
  ];

  tasks = {
    "dx:build" = {
      description = "Build the project";
      exec = "${lib.getExe pkgs.dioxus-cli} bundle";
    };
    "dx:test" = {
      description = "Run tests";
      exec = "${lib.getExe pkgs.dioxus-cli} check";
    };
    "dx:run" = {
      description = "Run the application";
      exec = "${lib.getExe pkgs.dioxus-cli} run";
    };
    "dx:serve" = {
      description = "Serve the application";
      exec = "${lib.getExe pkgs.dioxus-cli} serve";
    };
    "dx:format" = {
      description = "Format the project";
      exec = "${lib.getExe pkgs.dioxus-cli} fmt";
    };
    "cargo:test" = {
      description = "Run cargo tests";
      exec = "${lib.getExe pkgs.cargo} test";
    };
    "cargo:deps-test" = {
      description = "Run cargo tests with dependencies";
      exec = "${lib.getExe pkgs.cargo} test --all-features";
    };
  };

  languages.rust.enable = true;

  enterShell = ''
    ${lib.getExe pkgs.git} --version
    ${lib.getExe pkgs.dioxus-cli} --version
    ${lib.getExe pkgs.cargo} --version
  '';

  enterTest = ''
    echo "Dioxus Format"
    ${config.tasks."dx:format".exec}
    echo "Dioxus Test"
    ${config.tasks."dx:test".exec}
    echo "Cargo Test"
    ${config.tasks."cargo:test".exec}
    echo "Cargo Dependencies Test"
    ${config.tasks."cargo:deps-test".exec}
  '';

  git-hooks.hooks = {
    action-validator.enable = true;
    actionlint.enable = true;
    check-added-large-files.enable = true;
    check-builtin-literals.enable = true;
    check-case-conflicts.enable = true;
    check-json.enable = true;
    check-merge-conflicts.enable = true;
    check-toml.enable = true;
    check-vcs-permalinks.enable = true;
    check-xml.enable = true;
    check-yaml.enable = true;
    clippy.enable = true;
    comrak.enable = true;
    deadnix.enable = true;
    detect-private-keys.enable = true;
    # markdownlint.enable = true;
    mixed-line-endings.enable = true;
    nixfmt.enable = true;
    prettier.enable = true;
    ripsecrets.enable = true;
    rustfmt.enable = true;
    statix.enable = true;
    taplo.enable = true;
    trim-trailing-whitespace.enable = true;
    trufflehog.enable = true;
    # yamllint.enable = true;
  };

  treefmt = {
    enable = true;
    config.programs = {
      rustfmt.enable = true;
      nixfmt.enable = true;
    };
  };
}
