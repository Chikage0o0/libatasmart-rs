{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  packages = [
    # --- 版本控制 ---
    pkgs.git # Git 版本控制
    pkgs.git-lfs # Git 大文件存储

    # --- 代码格式化 ---
    pkgs.nixfmt # Nix 代码格式化工具

    # --- Shell 和终端工具 ---
    pkgs.bashInteractive # 交互式 Bash shell
    pkgs.bash-completion # Bash 自动补全
    pkgs.ncurses # 终端界面库
    pkgs.util-linux # Linux 系统工具集
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
  };

  # https://devenv.sh/processes/
  # processes.dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  # https://devenv.sh/basics/
  enterShell = ''
    hello         # Run scripts directly
    git --version # Use packages
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
