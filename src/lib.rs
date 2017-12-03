mod day1;
mod day2;
mod day3;

pub struct Config {
    pub day: u32,
    pub part: u32,
    pub extra: Option<u32>,
}

impl Config {
    fn parse_args(args: &[String])-> Result<Config, &'static str> {
       let day = 
           match u32::from_str_radix(&args[1], 10) {
               Ok(res) => res,
               Err(_) => return Err("Invalid day"),
           };

       let part = 
           match u32::from_str_radix(&args[2], 10) {
               Ok(res) => res,
               Err(_) => return Err("Invalid part"),
           };

       let extra = if args.len() > 3 {
           match u32::from_str_radix(&args[3], 10) {
               Ok(res) => Some(res),
               _ => None, 
           }
        } else {
            None
        };

       Ok( Config {day, part, extra})
    }


    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            Err("Not enough args")
        }
        else {
            Self::parse_args(args)
        }

    }

}

pub fn dispatch(config: &Config) {
    match config.day {
        1 => day1::run(config.part),
        2 => day2::run(config.part),
        3 => day3::run(config.part, config.extra.unwrap()),
        _ => println!("NYI!"),
    }
}



