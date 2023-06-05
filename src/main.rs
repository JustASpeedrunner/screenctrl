use eframe::egui;
use std::process::Command;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let bmax = Command::new("brightnessctl").arg("m").output().expect("Could not get max monitor brightness.");
    let bnm = String::from_utf8(bmax.stdout).expect("invalid utf8");
    println!("{:?}", bnm);
    let brightnessmax = bnm.trim().parse::<i32>().unwrap();
    let boffset = brightnessmax/100;

    // Actual fucking spaghetti code, idk if this will work for anyone else so please pr with sensible code :pray:
    let bcurrout = Command::new("brightnessctl").arg("g").output().expect("Could not get current monitor brightness.");
    let bn = String::from_utf8(bcurrout.stdout).expect("invalid utf8");
    let brightness = bn.trim().parse::<i32>().unwrap();

    let mut brightnessslider = brightness/boffset;

    eframe::run_simple_native("Screenctrl", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Monitor controls");
            ui.add(egui::Slider::new(&mut brightnessslider, 0..=100).text("Brightness"));
        });
        let _ = Command::new("brightnessctl").arg("s").arg((brightnessslider*boffset).to_string()).spawn();
    })
}