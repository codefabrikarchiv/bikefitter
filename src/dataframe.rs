use std::fmt;
use std::str::FromStr;

#[derive(Default, Copy, Clone)]
pub struct Dataframe {
    pub x: i32,
    pub y: i32,
    pub action: i32,
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

pub struct ParserError;

impl FromStr for Dataframe {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("to frame: {}", s);
        let t = s.trim();
        let parts: Vec<&str> = t.split('#').collect();
        if parts.len() == 3 {
            let x = match parts[0].to_string().parse::<i32>() {
                Ok(x) => x,
                Err(_err) => return Err(ParserError)
            };
            let y = match parts[1].to_string().parse::<i32>() {
                Ok(y) => y,
                Err(_err) => return Err(ParserError)
            };
            let action = match parts[2].to_string().parse::<i32>() {
                Ok(action) => action,
                Err(_err) => return Err(ParserError)
            };

            Ok(Dataframe {
                x,
                y,
                action
            })
        } else {
            Err(ParserError)
        }
        
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
        let str3 = "9104#10208#0\n";
        let df3 = Dataframe { x: 9104, y: 10208, action: 0 };
        assert_eq!(Dataframe::from_str(str3).unwrap(), df3);
    }
}
