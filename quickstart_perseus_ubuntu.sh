## Exit on error. ##
set -e
## Update package lists. ##
sudo apt-get update
## Install some system lib/tool dependencies for perseus build. ##
sudo apt-get install -y build-essential libssh-dev openssl pkg-config
## See if rustup is on PATH. ##
if ! which rustup
  then
    ## If rustup is in "~/.cargo/bin". ##
    if [ -f "/home/${USER}/.cargo/bin/rustup" ]
      then
        ## Update PATH. ##
        PATH="/home/${USER}/.cargo/bin:${PATH}"
      else
        ## Install RustUp. ##
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      fi
  fi
## Update rust toolchain and rustup. ##
rustup update
## Install Perseus CLI. ##
cargo install perseus-cli
## Only generate a new project if not already inside of an existing project. ##
if [ ! "${PWD##*/}" = "rust-perseus-app-basic" ] || [ ! -f "Cargo.toml" ]
  then
    ## Use Perseus CLI to build new project. ##
    perseus new  quickstart-perseus
    ## Enter project directory. ##
    cd quickstart-perseus
  fi
## Serve Perseus project (includes workaround for versions below v0.4.3). ##
cli_version="$(perseus --version | awk '{print $2}' | tr -d '.')"
if [ "043" -gt "${cli_version}" ]
  then
    perseus --wasm-opt-version version_118 serve -w
  else
    perseus serve -w
  fi
