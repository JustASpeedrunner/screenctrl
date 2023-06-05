# Screenctrl
A GUI tool to manage your monitor's brightness, using brightnessctl, and color temprature, using xsct, in Xorg.

Note: This is my first time using egui or even making a gui. Please excuse the simplistic/poor styling, it will improve after core functionality is completed.

## SCREEN TEMPERATURE IS CURRENTLY WIP, IT IS NOT IMPLEMENTED YET
### Todo list:
- Test multi-monitor support
- Add screen temperature
- (Maybe) add dual sliders for individual monitor control.

## How to install
Dependencies:
- Xorg
- Rust (Cargo)
- brightnessctl
- xsct

Installing Rust (rustup): ``curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``

Arch (using AUR helper): ``yay -S xorg brightnessctl xsct``

Other distros: Check your repositories for the corresponding packages, I cannot test on other distros so if you encounter any errors please make an issue.
