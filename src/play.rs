use std::str::FromStr;

pub struct Play {
    pub x: usize,
    pub y: usize,
}

pub enum ParsePlayError {
    BadLen,
    ParseInt
}

impl FromStr for Play {
    type Err = ParsePlayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        let (Some(x), Some(y), None) = (split.next(), split.next(), split.next()) else {
            return Err(ParsePlayError::BadLen);
        };

        let x = x.parse().map_err(|_| ParsePlayError::ParseInt)?;
        let y = y.parse().map_err(|_| ParsePlayError::ParseInt)?;


        Ok(Self { x, y })
    }
}