set -e
set -u

if [ -f .env ]; then
    export $(cat .env | xargs)
else
    echo ".env file not found"
fi

docker run -e GIT_CREDENTIALS --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.6