# To be run in linux container running rusty-ci!
cd ~
git clone https://github.com/adam-mcdaniel/rusty-ci
cd rusty-ci
git pull origin master

cargo install -f --path .
. venv/bin/activate

rusty-ci build -q rusty_ci.yaml
rusty-ci start -q rusty_ci.yaml