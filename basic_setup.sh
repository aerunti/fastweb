sudo apt purge postgresql-14* postgresql-client-14
sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
wget -qO- https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo tee /etc/apt/trusted.gpg.d/pgdg.asc &>/dev/null
apt update && apt dist-upgrade
sudo apt autoremove
sudo apt install build-essential libssl-dev libpq-dev postgresql-15 postgresql-client-15 redis pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
cargo install cargo-edit
rustup component add clippy

sudo -u postgres psql -c "CREATE ROLE fastweb SUPERUSER LOGIN PASSWORD 'fastweb';"






source "$HOME/.cargo/env"
cargo install cargo-edit
rustup component add clippy
sudo apt install libssl-dev
cargo install cargo-tarpaulin
cargo install cargo-watch
cargo install sqlx-cli
sudo service postgresql start
sudo -u postgres psql -c "create database fastweb;"
sqlx migrate run
bash make_watch.sh
