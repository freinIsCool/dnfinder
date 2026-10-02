# dnfinder
a rataTUI frontent for dnf, made for fedora. Inspired by omarchies AUR frontend
built with rust and ratatui

installing:

1. download the ``dnfnd`` binary from 'placeholder'
   option 1 (single user):
2. copy it to $HOME/.local/bin/
3. run ``dnfnd`` in a terminal
   option 2 (global):
2. copy it to /usr/bin/share/
3. run ``dnfnd`` in a terminal


building from source

requirements:

* cargo
* rust
* ratatui
* ratatui-textarea
* color-eyre
* crossterm

```bash
# 1. clone the repo
git clone https://github.com/freinIsCool/dnfinder.git

# 2. build
cd cargo/
cargo build

# 3. run
./target/debug/dnfnd
```
   
