# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.205.2/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>


# create a volume for cargo compiles to use
# by default this will go into the same folder as the project, but this
# isn't ideal if that folder is mounted over hgfs osx->linux as in that 
# case it cannot create hardlinks for already-compiled artifacts
# in which case, it's a lot slower
ENV CARGO_TARGET_DIR="/target"
VOLUME ["/target"]

# vscode user
RUN mkdir /target && chown 1000:1000 /target


