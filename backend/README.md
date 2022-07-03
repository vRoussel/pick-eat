The "build_for_deploy.sh" script will build a statically linked binary, using the x86_64-unknown-linux-musl target.
For that, you need to install a few things beforehand:
 - rustup target add x86_64-unknown-linux-musl
 - some MUSL dependencies, cargo will guide you ('musl' package was enough for me on arch)
