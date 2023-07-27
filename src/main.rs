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
        // If you have a better way to do this then please pr, idk what I'm doing.
    let bcurrout = Command::new("brightnessctl").arg("g").output().expect("Could not get current monitor brightness.");
    let bn = String::from_utf8(bcurrout.stdout).expect("invalid utf8");
    let brightness = bn.trim().parse::<i32>().unwrap();


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
    // This rounds sliderval to the nearest 5%, it's not the prettiest thing but it works.
    if sliderval%5 == 0 {
        return sliderval
    } else {
        if (sliderval-2)%5 == 0 {
            return sliderval-2
        } else if (sliderval-1)%5 == 0 {
            return sliderval-1
        } else if (sliderval+2)%5 == 0 {
            return sliderval+2
        } else if (sliderval+1)%5 == 0 {
            return sliderval+1
        } else {panic!("you set the slider value to a non-number")}
    }
}