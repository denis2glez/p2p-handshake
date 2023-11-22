## 🔎 Content

- [🔎 Content](#-content)
- [🤔 About ](#-about-)
- [🏁 Getting Started ](#-getting-started-)
- [🔧 Development ](#-development-)
  - [Quick check ](#quick-check-)
  - [Build ](#build-)
  - [Run tests ](#run-tests-)
- [🚀 Deployment ](#-deployment-)
- [🎉 Acknowledgements ](#-acknowledgements-)
- [📝 License ](#-license-)

## 🤔 About <a name = "about"></a>

This repo began as an exploration of a simple implementation of a [TLS] handshake on a peer-to-peer
(P2P) network and led to a more in-depth study of the popular P2P networking framework [libp2p], and,
in particular, of the Rust implementation [rust-libp2p].

> [!IMPORTANT]
> This repository is still a work in progress and includes code that is
> [being merged](https://github.com/libp2p/rust-libp2p/pull/4864) into the upstream rust-libp2p project.

For futher information, see the [docs](./docs/overview.md).

## 🏁 Getting Started <a name = "getting-started"></a>

To use your host system as development enviroment install the following dependencies.

- `curl`, `git`
- [protobuf compiler](https://grpc.io/docs/protoc-installation)
- [Rust](https://www.rust-lang.org/tools/install).
- [Docker](https://docs.docker.com/get-docker/).

> [!TIP]
> In any case, you can check below for suggestions on how to install the prerequisites on your system.

<details open>
<summary><b>Linux (Debian/Ubuntu)</b></summary>

If you are using Debian or a derivative (e.g. Ubuntu, Linux Mint), it is recommended to install Rust
using the standard installation script. You could install all the development dependencies by running
the following commands.
```sh
sudo apt install curl git docker protobuf-compiler
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
</details>

<details close>
<summary><b>macOS</b></summary>

If you are using macOS you could install all the development dependencies using [Homebrew](https://brew.sh)
by running the following commands.
```sh
brew install curl git docker protobuf
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
</details>

<details close>
<summary><b>Windows</b></summary>

If you are using Windows, you could install most of the required dependencies using the
[`winget`](https://docs.microsoft.com/en-us/windows/package-manager/winget/#production-recommended)
CLI tool by running the following commands.
```sh
winget install --id Git.Git
winget install --id Docker.DockerDesktop
winget install --id Rustlang.Rust.MSVC
```
> You can probably run the shell scripts on your Windows system if you use Git Bash, but it is recommended
> to use the Windows Subsystem for Linux ([WSL](https://docs.microsoft.com/en-us/windows/wsl/install))
> instead as development environment and follow the suggestions for Debian/Ubuntu.
</details>

## 🔧 Development <a name = "development"></a>

Once you have a development environment configured with all the necessary dependencies, you can
perform any of the following tasks.

### Quick check <a name = "quick-check"></a>

Quickly check the package and all of its dependencies for possible errors
```sh
cargo check
```

### Build <a name = "build"></a>

To build the packages use
```sh
cargo build
```

### Run tests <a name = "run-tests"></a>

First we need to start a private Celestia network with single validator and bridge
```sh
docker compose -f docker/docker-compose.yml up --build --force-recreate -d
```
We then generate the authentication tokens that will be used by the tests
```sh
scripts/gen_auth_tokens.sh
```
Now we can run all the default tests
```sh
cargo test
```
or just a specific group of tests, by adding `-- <pattern>` to filter.

To conclude, shutdown the private Celestia network
```sh
docker compose -f docker/docker-compose.yml down
```

## 🚀 Deployment <a name = "deployment"></a>

You can use any of the tarballs in the [Releases section](https://github.com/denis2glez/p2p-handshake/releases)
to deploy the software according to your requirements. These are automatically generated using the
release workflow after tagging a new version.

## 🎉 Acknowledgements <a name = "acknowledgement"></a>

Thanks to all the developers of the libraries used throughout the project.

## 📝 License <a name = "license"></a>

This project is licensed under the [MIT](LICENSE) license.

[TLS]: https://datatracker.ietf.org/doc/rfc8446
[libp2p]: https://docs.libp2p.io/
[rust-libp2p]: https://github.com/libp2p/rust-libp2p