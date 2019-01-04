use detectors_rs::surface::Config;
use nalgebra::Point3;
use rayon::prelude::*;
use std::{sync::Mutex, path::PathBuf, fs::File};
use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "detectors",
    about = "A program to calculate detector properties.",
    version="",
    author = "",
    raw(global_settings = "&[AppSettings::DisableVersion]")
)]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn theta(p: Point3<f64>) -> f64 {
    f64::acos(p[2] / f64::sqrt(p[0].powi(2) + p[1].powi(2) + p[2].powi(2))).to_degrees()
}

fn phi(p: Point3<f64>) -> f64 {
    let mut phi = f64::atan2(p[0], p[1]).to_degrees();
    if phi < 0.0 { phi += 360.0 };
    phi
}

fn main() -> Result<(), Box<std::error::Error>> {
    let opt = Opt::from_args();
    let mut config = Config::new();
    for file_path in opt.files {
        let file = File::open(file_path)?;
        config.add_from_reader(file)?;
    }
    let dets = config.to_detectors();
    let output = Mutex::new(Vec::new());
    dets.par_iter().enumerate().for_each(|(i, d)| {
        d.par_iter().enumerate().for_each(|(j, s)| {
            output.lock().unwrap().push((i + 1, j,
                                         s.func_min(theta), s.func_max(theta), s.func_avg(theta),
                                         s.func_min(phi), s.func_max(phi), s.func_avg(phi),
                                         s.solid_angle()));
        });
    });

    let mut output = output.into_inner().unwrap();
    output.sort_by_key(|x| x.1);
    output.sort_by_key(|x| x.0);

    for (det, chan, th_min, th_max, th_avg, phi_min, phi_max, phi_avg, solid_angle) in output {
        println!("{}\t{}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{}", det, chan, th_min, th_max, th_avg, phi_min, phi_max, phi_avg, solid_angle);
    }
    Ok(())
}
