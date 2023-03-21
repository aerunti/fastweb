#!/usr/bin/bash
sudo service postgresql start
sudo service redis-server start
# cargo watch -x check -x test -x run
cargo  watch -x run