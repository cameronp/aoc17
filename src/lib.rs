mod week1;

pub struct Config {
    pub week: u32,
    pub part: u32,
}

impl Config {
    fn parse_args(week_s: &str, part_s: &str) -> Result<Config, &'static str> {
       let week = u32::from_str_radix(week_s, 10).unwrap();
       let part = u32::from_str_radix(part_s, 10).unwrap();

       Ok( Config {week: week, part: part})
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
    match config.week {
        1 => week1::run(config.part),
        _ => println!("NYI!"),
    }
}



