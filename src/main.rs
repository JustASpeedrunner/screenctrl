use eframe::egui;
use std::process::Command;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    // Start brightness related declarations
    let bmax = Command::new("brightnessctl").arg("m").output().expect("Could not get max monitor brightness.");
    let bnm = String::from_utf8(bmax.stdout).expect("invalid utf8");
    let brightnessmax = bnm.trim().parse::<i32>().unwrap();
    let boffset = brightnessmax/100;
        // If you have a better way to do this then please pr, idk what I'm doing.
    let bcurrout = Command::new("brightnessctl").arg("g").output().expect("Could not get current monitor brightness.");
    let bn = String::from_utf8(bcurrout.stdout).expect("invalid utf8");
    let brightness = bn.trim().parse::<i32>().unwrap();
    // End brightness related declarations

    let mut brightnessslider = brightness/boffset;
    let mut round = false;
    eframe::run_simple_native("Screenctrl", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Monitor controls");
            ui.add(egui::Slider::new(&mut brightnessslider, 0..=100).text("Brightness"));
            ui.checkbox(
                &mut round,
                format!(
                    "Round slider value to nearest 5%",
                ),
            );
        });
        match round {
            false => {let _ = Command::new("brightnessctl").arg("s").arg((brightnessslider*boffset).to_string()).spawn();},
            true => {
                {
                    let _ = Command::new("brightnessctl").arg("s").arg((rounding(brightnessslider)*boffset).to_string()).spawn();
                    brightnessslider = rounding(brightnessslider);
                }
            },
        }
    })
}

fn rounding(sliderval:i32) -> i32 {
    for num in -2..=2 {
        if (sliderval+num)%5 == 0 {
            return sliderval+num
        }
    }
}