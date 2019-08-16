#!/bin/bash
# Run in linux container running rusty-ci!
# To use this, run the install, build, and start subcommands first.
. ~/.bashrc    # Source .bashrc
. ~/.cargo/env # Get access to installed crates

cd ~
git clone https://github.com/adam-mcdaniel/rusty-ci
cd rusty-ci
git reset --hard # Reset so we can pull!
git pull         # Get new release


cargo install -f --path . # Install the new release 

# Hot reload rusty-ci with new release
. venv/bin/activate
rusty-ci rebuild -q rusty_ci.yaml
rusty-ci start -q rusty_ci.yaml
