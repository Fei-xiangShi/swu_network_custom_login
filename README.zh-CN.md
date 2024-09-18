# 西南大学校园网自定义登录客户端

这是一个为西南大学（SWU）定制的网络登录客户端，能够自动检测网络连接状态并在需要时执行登录操作。该客户端由Rust编写，支持多种平台，包括Windows、macOS、Linux以及适用于OpenWRT设备的各种架构。

## 介绍文档语言

en-US [English](https://github.com/Fei-xiangShi/swu_network_custom_login?tab=readme-ov-file#swu-network-custom-login-client "English version readme")

zh-CN [简体中文](https://github.com/Fei-xiangShi/swu_network_custom_login/blob/main/README.zh-CN.md "简体中文版简介")

## 功能

- **自动网络检测**：持续监控网络状态，当网络连接中断时自动执行登录操作。
- **多平台支持**：提供适用于多种平台和架构的预编译二进制文件。
- **密码加密**：支持加密密码传输以增强安全性。
- **可定制化**：通过 `accounts.txt`文件轻松配置账号信息。
- **轻量级**：占用资源少，适用于能力有限的设备。

## 目录

- [安装](#安装)

  - [预编译二进制文件](#预编译二进制文件)
  - [从源码构建](#从源码构建)
- [配置](#配置)
- [使用](#使用)
- [支持的平台](#支持的平台)
- [贡献](#贡献)
- [许可证](#许可证)

## 安装

### 预编译二进制文件

从[Releases](https://github.com/Fei-xiangShi/swu_network_custom_login/releases)页面下载适合您平台的预编译二进制文件。

**可用平台：**

- **Windows**

  - x86_64 (`x86_64-pc-windows-msvc`)
  - x86 (`i686-pc-windows-msvc`)
  - ARM64 (`aarch64-pc-windows-msvc`)
- **macOS**

  - x86_64 (`x86_64-apple-darwin`)
  - ARM64 (`aarch64-apple-darwin`)
- **Linux**

  - x86_64 (`x86_64-unknown-linux-gnu`)
  - x86 (`i686-unknown-linux-gnu`)
  - ARM64 (`aarch64-unknown-linux-gnu`)
  - ARMV7 (`armv7-unknown-linux-gnueabihf`)
  - PowerPC64 (`powerpc64le-unknown-linux-gnu`)
  - RISC-V64 (`riscv64gc-unknown-linux-gnu`)
  - S390x (`s390x-unknown-linux-gnu`)
- **FreeBSD**

  - x86_64 (`x86_64-unknown-freebsd`)
- **Illumos**

  - x86_64 (`x86_64-unknown-illumos`)

**步骤：**

1. 下载适合你平台的 `.tar.gz`文件。
2. 解压归档文件：

   ```bash
   tar -xzvf swu_network_custom_login.tar.gz
   ```
3. 将可执行文件放置在 `PATH`目录中，或直接运行它。

### 从源码构建

确保已安装[Rust](https://www.rust-lang.org/tools/install)。

1. 克隆仓库：
   ```bash
   git clone https://github.com/Fei-xiangShi/swu_network_custom_login.git
   ```
2. 进入项目目录：
   ```bash
   cd swu_network_custom_login
   ```
3. 构建项目：
   ```bash
   cargo build --release
   ```
4. 编译后的二进制文件位于 `target/release/`目录中。

## 配置

在与可执行文件相同的目录下创建一个 `accounts.txt`文件，文件中应包含你的登录凭据。

**格式：**

- 每一行代表一个账号。
- 用户名和密码由空格分隔。

**示例：**

```
student123 mypassword
user456 anotherpassword
```

**注意**：如果找不到 `accounts.txt`文件，程序将创建一个包含示例账号的文件并提示你进行编辑。

## 使用

只需运行可执行文件。程序将自动：

1. 定期检查网络连接状态。
2. 如果网络断开，它会尝试获取登录页面的URL。
3. 如有需要，加密密码。
4. 使用 `accounts.txt`中的凭据执行登录操作。

**运行程序：**

```bash
./swu_network_custom_login
```

**日志输出：**

程序使用 `env_logger`库进行日志记录。你可以通过设置 `RUST_LOG`环境变量来控制日志级别。

**示例：**

- 将日志级别设置为 `info`：

  ```bash
  export RUST_LOG=info
  ```
- 运行程序：

  ```bash
  RUST_LOG=info ./swu_network_custom_login
  ```

## 支持的平台

客户端支持多种平台和架构，适用于各种设备，包括运行OpenWRT的路由器。

**支持的平台与架构：**

- **Windows**：x86, x86_64, ARM64
- **macOS**：x86_64, ARM64 (Apple Silicon)
- **Linux**：x86, x86_64, ARM64, LoongArch64, MIPS64, PowerPC64, RISC-V64, S390x

## 贡献

欢迎贡献！请按照以下步骤进行：

1. Fork该仓库。
2. 创建新分支：

   ```bash
   git checkout -b feature/your_feature
   ```
3. 提交你的更改：

   ```bash
   git commit -am 'Add a new feature'
   ```
4. 推送到分支：

   ```bash
   git push origin feature/your_feature
   ```
5. 创建Pull Request。

请确保你的代码符合现有风格并包含适当的测试。

## 许可证

本项目根据[GPL v2.0许可证](https://www.gnu.org/licenses/old-licenses/gpl-2.0.txt "GPL v2.0")授权。

---

**免责声明**：本项目仅用于教育目的。请确保您拥有在网络环境中使用它的必要权限。风险自负。
