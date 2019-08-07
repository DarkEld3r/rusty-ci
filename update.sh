#! /bin/bash
# Run in linux container running rusty-ci!
# To use this, run rusty-ci install first.

. ~/.bashrc
. ~/.cargo/env

cd ~
git clone https://github.com/adam-mcdaniel/rusty-ci
cd rusty-ci
git reset --hard
git pull


cargo install -f --path .

. venv/bin/activate

rusty-ci rebuild -q rusty_ci.yaml