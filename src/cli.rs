use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Config {
    // Camera settings
    pub aspect_ratio: f64,
    pub fov_radians: f64,
    // Output settings
    pub image_height: usize,
    // Quality settings
    pub rays_per_pixel: usize,
    pub max_scatter_depth: usize,
}

pub fn cli() -> clap::Command {
    clap::Command::new("crayfish")
       .author("Vlad Mikulik <vv@mikulik.me>")
       .about("A Rust raytracer.")
       .group(clap::ArgGroup::new("camera_settings").multiple(true))
       .next_help_heading("CAMERA SETTINGS")
       .arg(
           clap::Arg::new("aspect_ratio")
              .long("aspect_ratio")
              .value_names(["WIDTH", "HEIGHT"])
              .num_args(2)
              .help("Aspect ratio of the picture.")
              .default_values(["16", "9"])
              .value_parser(clap::value_parser!(f64))
              .group("camera settings")
       )
       .arg(
           clap::Arg::new("fov")
           .long("fov")
           .help("Field of View in degrees.")
           .default_value("90")
           .value_parser(clap::value_parser!(f64))
           .group("camera_settings")
       )
       .group(clap::ArgGroup::new("output_settings").multiple(true))
       .next_help_heading("OUTPUT SETTINGS")
       .arg(
            clap::Arg::new("image_height")
            .long("image_height")
            .help("Output image height in pixels.")
            .default_value("100")
            .value_parser(clap::value_parser!(usize))
            .group("output_settings")
       )
       .group(clap::ArgGroup::new("quality_settings").multiple(true))
       .next_help_heading("QUALITY SETTINGS")
       .arg(
            clap::Arg::new("rays_per_pixel")
            .long("rays_per_pixel")
            .help("Number of rays to cast per pixel.")
            .default_value("200")
            .value_parser(clap::value_parser!(usize))
            .group("quality_settings")
       )
        .arg(
            clap::Arg::new("max_scatter_depth")
            .long("max_scatter_depth")
            .help("Maximum number of ray bounces.")
            .default_value("30")
            .value_parser(clap::value_parser!(usize))
            .group("quality_settings")
       )
}

pub fn make_config(matches: clap::ArgMatches) -> Result<Config, Box::<dyn Error>> {
    let wh: Vec<&f64> = matches.get_many("aspect_ratio").unwrap().collect();
    let aspect_ratio = wh[0] / wh[1];

    Ok(Config {
        aspect_ratio,
        fov_radians: matches.get_one::<f64>("fov").unwrap().to_radians(),
        image_height: *matches.get_one("image_height").unwrap(),
        rays_per_pixel: *matches.get_one("rays_per_pixel").unwrap(),
        max_scatter_depth: *matches.get_one("max_scatter_depth").unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_args_as_expected() {
        let cmd = cli();
        let matches = cmd.get_matches_from("crayfish".split(' '));
        let config = make_config(matches).unwrap();
        assert_eq!(
            config,
            Config {
                aspect_ratio: 16./9.,
                fov_radians: std::f64::consts::PI / 2.,
                image_height: 100,
                rays_per_pixel: 200,
                max_scatter_depth: 30,
            }
        )
    }
}