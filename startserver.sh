#!/bin/bash
/var/stuff/app/redis-stable/src/redis-server --daemonize yes
cd /var/stuff/app/var/stuff/app
cargo run
