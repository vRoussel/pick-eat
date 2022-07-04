export DOCKER_BUILDKIT=1
script_dir="$(dirname "$(realpath "$0")")"
out_dir="../ansible/to_deploy/backend"

echo "Building for arm64"
(cd "$script_dir"; docker build --platform linux/arm64 -o ${out_dir}/aarch64/ .)

echo "Building for amd64"
(cd "$script_dir"; docker build --platform linux/amd64 -o ${out_dir}/x86_64/ .)
