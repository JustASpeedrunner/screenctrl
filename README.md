# [Screenctrl](https://github.com/JustASpeedrunner/screenctrl)
A GUI tool to manage your monitor's brightness, using brightnessctl, and color temprature, using xsct, in Xorg.

**Note: This is my first time using egui or even making a gui. Please excuse the simplistic/poor styling, it will improve after core functionality is completed.**

**Also let it be known I will not be making consistent updates to this, as I find time and motivation I will but not always. If you'd like to contribute submit a PR.**

### Todo list:
- Fix multi-monitor support ***(Currently borked)***
- Add screen temperature
- (Maybe) add dual sliders for individual monitor control.

## How to install
Dependencies:
- Xorg
- Rust (Cargo)
- [brightnessctl](https://github.com/Hummer12007/brightnessctl)
- [xsct](https://github.com/faf0/sct)

Installing rustup (distro independent): ``curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``

Arch (using AUR helper): ``yay -S xorg brightnessctl xsct``

Other distros: Check your repositories for the corresponding packages, or install them from source. I cannot test on other distros so if you encounter any errors please make an [issue](https://github.com/JustASpeedrunner/screenctrl/issues).

Running the project:
```
git clone https://github.com/JustASpeedrunner/screenctrl
cd screenctrl
cargo run --release
```

Note: it does still print brightnessctl outputs to the terminal if you launch it from one. If this is an issue, launch with ``nohup cargo run --release``

Once built, you may move the resulting binary (located at ``target/release/screenctrl``) wherever you like. If you'd like it to be launchable from anywhere, you can place it in your ``/usr/bin/`` or similar.

## Find the program useful? Want to help me be able to develop it faster?
Donate to me via CashApp or Monero!

Cashtag: $66lol99

Monero: 46ExBswshFq2H4rWoAmY7m1suGNzbmLvk5WE6ZsLhmFy17L8KkDkSrmD673GS8CBZh962RDVVokfdAnJ98mVA4yB5h5DNXQ

Donations are not required but are greatly appreciated! If you donate your name will be listed at the bottom of this file once your donation is confirmed. To confirm your donation(s), join the [discord](https://discord.gg/gndKuB92u4) and navigate to the "sys.screenctrl" category.