# SWU Network Custom Login Client

A custom network login client for Southwest University (SWU) that automatically detects network connectivity and performs login operations when necessary. This client is written in Rust and supports multiple platforms, including Windows, macOS, Linux, and various architectures suitable for OpenWRT devices.

## Readme Language

en-US [English](#readme-language "English version readme")

zh-CN [简体中文](https://github.com/Fei-xiangShi/swu_network_custom_login/blob/main/README.zh-CN.md "简体中文版简介")

## Features

- **Automatic Network Detection**: Continuously monitors network status and initiates login when connectivity is lost.
- **Multi-Platform Support**: Precompiled binaries available for a wide range of platforms and architectures.
- **Password Encryption**: Supports encrypted password transmission for enhanced security.
- **Customizable**: Easy configuration through the `accounts.txt` file.
- **Lightweight**: Minimal resource usage, suitable for devices with limited capabilities.

## Table of Contents

- [Installation](#installation)
  - [Precompiled Binaries](#precompiled-binaries)
  - [From Source](#from-source)
- [Configuration](#configuration)
- [Usage](#usage)
- [Supported Platforms](#supported-platforms)
- [Contributing](#contributing)
- [License](#license)

## Installation

### Precompiled Binaries

Download the precompiled binary for your platform from the [Releases](https://github.com/your_username/your_repository/releases) page.

**Available Platforms:**

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
  - LoongArch64 (`loongarch64-unknown-linux-gnu`)
  - MIPS64 (`mips64-unknown-linux-gnuabi64`)
  - PowerPC64 (`powerpc64le-unknown-linux-gnu`)
  - RISC-V64 (`riscv64gc-unknown-linux-gnu`)
  - S390x (`s390x-unknown-linux-gnu`)

**Steps:**

1. Download the appropriate `.tar.gz` file for your platform.
2. Extract the archive:
   ```bash
   tar -xzvf swu_network_custom_login.tar.gz
   ```
3. Place the executable in a directory included in your `PATH`, or run it directly.

### From Source

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

1. Clone the repository:
   ```bash
   git clone https://github.com/Fei-xiangShi/swu_network_custom_login.git
   ```
2. Navigate to the project directory:
   ```bash
   cd swu_network_custom_login
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The compiled binary will be located in `target/release/`.

## Configuration

Create an `accounts.txt` file in the same directory as the executable. This file should contain your login credentials.

**Format:**

- Each line represents a separate account.
- Username and password are separated by a space.

**Example:**

```

student123 mypassword

user456 anotherpassword

```

**Note:** If the `accounts.txt` file is not found, the program will create one with an example account and prompt you to edit it.

## Usage

Simply run the executable. The program will automatically:

1. Check network connectivity at regular intervals.
2. If the network is down, it will attempt to retrieve the login page URL.
3. Encrypt the password if required.
4. Perform the login operation using the credentials from `accounts.txt`.

**Running the Program:**

```bash

./swu_network_custom_login

```

**Logging Output:**

The program uses the `env_logger` crate for logging. You can set the `RUST_LOG` environment variable to control the log level.

**Examples:**

- Set log level to `info`:

  ```bash

  export RUST_LOG=info

  ```
- Run the program:

  ```bash

  RUST_LOG=info./swu_network_custom_login

  ```

## Supported Platforms

The client supports a wide range of platforms and architectures, making it versatile for various devices, including routers running OpenWRT.

**Platforms and Architectures:**

- **Windows:** x86, x86_64, ARM64

- **macOS:** x86_64, ARM64 (Apple Silicon)

- **Linux:** x86, x86_64, ARM64, LoongArch64, MIPS64, PowerPC64, RISC-V64, S390x

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch:

   ```bash

   git checkout -b feature/your_feature

   ```
3. Commit your changes:

   ```bash

   git commit -am 'Add a new feature'

   ```
4. Push to the branch:

   ```bash

   git push origin feature/your_feature

   ```
5. Open a Pull Request.

Please ensure your code adheres to the existing style and includes appropriate tests.

## License

This project is licensed under the [GPL v2.0 License](https://www.gnu.org/licenses/old-licenses/gpl-2.0.txt "GPL v2.0").

---

**Disclaimer:** This project is intended for educational purposes. Please ensure you have the necessary permissions to use it within your network environment. Use at your own risk.
