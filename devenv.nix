{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # 基础软件包配置
  packages = [
    pkgs.git # Git 版本控制
    pkgs.git-lfs # Git 大文件存储
    pkgs.nixfmt # Nix 代码格式化
    pkgs.bashInteractive # 交互式 Bash
    pkgs.bash-completion # Bash 自动补全
    pkgs.ncurses # 终端界面库
    pkgs.util-linux # Linux 系统工具
    # C 项目构建工具
    pkgs.autoconf # Autoconf 配置工具
    pkgs.automake # Automake 构建工具
    pkgs.libtool # Libtool 库工具
    pkgs.pkg-config # pkg-config 依赖查找
    pkgs.gcc # GCC 编译器
    pkgs.systemd # 提供 libudev 库
  ];

  # Rust 语言支持
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [
      "x86_64-unknown-linux-musl"
    ];
  };

  # 进入 Shell 时的钩子
  enterShell = ''
    echo "Rust 互操作库开发环境已就绪"
    git --version
  '';

  # 测试钩子
  enterTest = ''
    echo "正在运行测试..."
    cargo test
  '';
}
