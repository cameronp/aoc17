mod day1;
mod day2;

pub struct Config {
    pub day: u32,
    pub part: u32,
}

impl Config {
    fn parse_args(day_s: &str, part_s: &str) -> Result<Config, &'static str> {
       let day = u32::from_str_radix(day_s, 10).unwrap();
       let part = u32::from_str_radix(part_s, 10).unwrap();

       Ok( Config {day: day, part: part})
    }


    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            Err("Not enough args")
        }
        else {
            Self::parse_args(&args[1], &args[2])
        }

    }

}

pub fn dispatch(config: &Config) {
    match config.day {
        1 => day1::run(config.part),
        2 => day2::run(config.part),
        _ => println!("NYI!"),
    }
}



