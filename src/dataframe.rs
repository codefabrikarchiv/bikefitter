use std::fmt;
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

impl fmt::Debug for Dataframe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{} ({})", self.x, self.y, self.action)
    }
}

impl PartialEq for Dataframe {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.action == other.action
    }
}

impl FromStr for Dataframe {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('#').collect();
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

#[cfg(test)]
mod tests {
    use crate::Dataframe;
    use std::str::FromStr;

    #[test]
    fn it_adds() {
        let df1 = Dataframe { x: 2, y: 3, action: 0 };
        let df2 = Dataframe { x: 4, y: 6, action: 0 };
        let df3 = Dataframe { x: 6, y: 9, action: 0 };
        assert_eq!(df1.add(df2), df3);
    }

    #[test]
    fn it_keeps_action_on_add() {
        let df1 = Dataframe { x: 2, y: 3, action: 1 };
        let df2 = Dataframe { x: 4, y: 6, action: 0 };
        let df3 = Dataframe { x: 6, y: 9, action: 1 };
        assert_eq!(df1.add(df2), df3);
    }

    #[test]
    fn it_subtracts() {
        let df1 = Dataframe { x: 2, y: 3, action: 0 };
        let df2 = Dataframe { x: 4, y: 6, action: 0 };
        let df3 = Dataframe { x: 2, y: 3, action: 0 };
        assert_eq!(df2.subtract(df1), df3);
    }

    #[test]
    fn it_keeps_action_on_subtract() {
        let df1 = Dataframe { x: 4, y: 6, action: 1 };
        let df2 = Dataframe { x: 2, y: 3, action: 0 };
        let df3 = Dataframe { x: 2, y: 3, action: 1 };
        assert_eq!(df1.subtract(df2), df3);
    }

    #[test]
    fn it_parses_from_string() {
        let str1 = "4#6#1";
        let df1 = Dataframe { x: 4, y: 6, action: 1 };
        assert_eq!(Dataframe::from_str(str1).unwrap(), df1);
        let str2 = "2#3#0";
        let df2 = Dataframe { x: 2, y: 3, action: 0 };
        assert_eq!(Dataframe::from_str(str2).unwrap(), df2);
    }
}
