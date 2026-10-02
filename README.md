# dnfinder
a rataTUI frontent for dnf, made for fedora. Inspired by omarchies AUR frontend
built with rust and ratatui


<img width="1877" height="1016" alt="dms_capture_1790944422420" src="https://github.com/user-attachments/assets/5f4dc1fb-2517-40ac-a688-1cdc8e4c049d" />


installing:

1. download the ``dnfnd`` binary from [here](https://github.com/freinIsCool/dnfinder/releases/tag/v1.0.0)

* option 1 (single user):
2. copy it to $HOME/.local/bin/
3. run ``dnfnd`` in a terminal
  
* option 2 (global):
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
   
