{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  packages = [ pkgs.dioxus-cli ];

  languages.rust.enable = true;

  enterShell = ''
    ${lib.getExe pkgs.git} --version
    ${lib.getExe pkgs.rustc} --version
    ${lib.getExe pkgs.cargo} --version
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
    markdownlint.enable = true;
    mixed-line-endings.enable = true;
    nixfmt.enable = true;
    prettier.enable = true;
    ripsecrets.enable = true;
    rustfmt.enable = true;
    statix.enable = true;
    taplo.enable = true;
    trim-trailing-whitespace.enable = true;
    trufflehog.enable = true;
    yamllint.enable = true;
  };

  treefmt = {
    enable = true;
    config.programs = {
      rustfmt.enable = true;
      nixfmt.enable = true;
    };
  };
}
