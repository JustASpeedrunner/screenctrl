use eframe::egui;
use eframe::IconData;
use core::panic;
use std::process::Command;
use std::env::var;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 250.0)),
        icon_data: Some(
            IconData::try_from_png_bytes(&include_bytes!("../images/icon.png")[..]).unwrap(),
        ),
        ..Default::default()
    };

    let wayland:bool;

    let waytest = var("WAYLAND_DISPLAY");
    match waytest {
        Ok(_s) => {wayland = true;},
        Err(e) => {
            println!("{}",e);
            wayland=false;
            let xtest = var("DISPLAY").unwrap();
            println!("{}",xtest);
        }
    }
    let bmax = Command::new("brightnessctl").arg("m").output().expect("Could not get max monitor brightness.");
    let bnm = String::from_utf8(bmax.stdout).expect("invalid utf8");
    let brightnessmax = bnm.trim().parse::<i32>().unwrap();
    let boffset = brightnessmax/100;
    let bcurrout = Command::new("brightnessctl").arg("g").output().expect("Could not get current monitor brightness.");
    let bn = String::from_utf8(bcurrout.stdout).expect("invalid utf8");
    let brightness = bn.trim().parse::<i32>().unwrap();
    
    let temp:i32;

    if wayland == false {
        let tcurrout = Command::new("xsct").output().expect("Couldn't get current screen temprature");
        let tn = String::from_utf8(tcurrout.stdout).expect("Invalid utf8");
        let v: Vec<&str> = tn.split(' ').collect();
        // this might not work if xsct changes the way they output shit so watch out for this
        temp = v[4].parse::<i32>().unwrap();
    } else {
        // No finding current screen temp as it cannot be gotten from wlsunset to my knowledge.
        temp = 6500;
    }

    let mut tempslider = temp;
    let mut brightnessslider = brightness/boffset;
    let mut round = false;

    eframe::run_simple_native("ScreenCtrl by JustASpeedrunner", options.clone(), move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(1.5);
            ui.heading("Monitor Controls");
            ui.separator();
            ui.add(egui::Slider::new(&mut brightnessslider, 0..=100).text("Brightness"));
            ui.checkbox(
                &mut round,
                format!(
                    "Round brightness slider to nearest 5%",
                ),
            );
            ui.add(egui::Slider::new(&mut tempslider, 700..=10000).text("Screen Temprature"));
            ui.label("Minimum screen temp: 700. Maximum: 10000.");
            ui.separator();
            ui.hyperlink_to("ScreenCtrl Github", "https://github.com/JustASpeedrunner/screenctrl");
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
        // This shit crashes the wayland session if you close and relaunch screenctrl 💀
        //match wayland {
        //    true => {
        //        if tempslider >= 6500 {
        //            let _ = Command::new("wlsunset").arg("-T").arg(tempslider.to_string()).spawn();
        //        } else if tempslider <= 6400 {
        //            let _ = Command::new("wlsunset").arg("-t").arg(tempslider.to_string()).spawn();
        //        }
        //    },
        //    false => {let _ = Command::new("xsct").arg(tempslider.to_string()).spawn();}
        //}
    })
}


fn rounding(sliderval:i32) -> i32 {
    for num in -2..=2 {
        if (sliderval+num)%5 == 0 {
            return sliderval+num
        }
    }
    panic!("for loop failed")
}