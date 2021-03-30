use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Default, Copy, Clone)]
pub struct Dataframe {
    pub x: i32,
    pub y: i32,
    pub action: i32,
}

impl Dataframe {
    pub fn subtract(&self, subtrahend: Dataframe) -> Dataframe {
        Dataframe {
            x: self.x - subtrahend.x,
            y: self.y - subtrahend.y,
            action: self.action
        }
    }

    pub fn add(&self, summand: Dataframe) -> Dataframe {
        Dataframe {
            x: self.x + summand.x,
            y: self.y + summand.y,
            action: self.action
        }
    }
}

impl FromStr for Dataframe {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();
        let x = parts[0].to_string().parse::<i32>()?;
        let y = parts[1].to_string().parse::<i32>()?;
        let action = parts[2].to_string().parse::<i32>()?;

        Ok(Dataframe {
            x,
            y,
            action
        })
    }
}
