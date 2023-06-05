# Screenctrl
A GUI tool to manage your monitor's brightness, using brightnessctl, and color temprature, using xsct, in Xorg.

**Note: This is my first time using egui or even making a gui. Please excuse the simplistic/poor styling, it will improve after core functionality is completed.**

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