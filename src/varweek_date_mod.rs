//! varweek_date_mod

use chrono::Datelike;
use std::fmt;
/// varweek_date format is fixed and global:  
/// 4 digit year has the unit c for CE common era, one space  
/// 2 digit for varweek (very similar to week) from 01v to 53v, has unit v, one space  
/// 1 digit for varweek-day from 1d (monday) to 7d(sunday)    
/// example: `2021c 53v 7d`  
pub struct VarweekDate {
    year: u32,
    varweek: u32,
    day: u32,
}

impl VarweekDate {
    /// constructor from year, varweek, day
    /// returns None on error
    /// # Example
    /// ```
    /// let varweeks = varweeks_millis::VarweekDate::from_yvd(2021,09,3).unwrap();
    /// assert_eq!(varweeks.to_string(),"2021c 09v 3d");
    /// ```
    pub fn from_yvd(year: u32, varweek: u32, day: u32) -> Option<Self> {
        if year < 1000 || year > 9999 || varweek < 1 || varweek > 53 || day < 1 || day > 7 {
            return None;
        }
        //return
        Some(VarweekDate { year, varweek, day })
    }

    /// constructor from year, ordinal_day
    /// returns None on error
    /// # Example
    /// ```
    /// let varweeks = varweeks_millis::VarweekDate::from_year_ordinal_day(2021,45).unwrap();
    /// assert_eq!(varweeks.to_string(),"2021c 07v 3d");
    /// ```
    pub fn from_year_ordinal_day(year: u32, ordinal_day: u32) -> Option<Self> {
        let varweek = ((ordinal_day as f64 - 0.1) / 7.0).floor() as u32 + 1;
        let day = ((ordinal_day as f64 - 0.1) % 7.0).round() as u32;
        // return
        Self::from_yvd(year, varweek, day)
    }

    /// constructor from NaiveDate
    /// # Example
    ///
    /// ```
    /// let nd = chrono::NaiveDate::from_ymd(2021,02,28);
    /// let varweeks = varweeks_millis::VarweekDate::from_naive_date(nd).unwrap();
    /// assert_eq!(varweeks.to_string() ,"2021c 09v 3d");
    /// ```
    pub fn from_naive_date(nd: chrono::NaiveDate) -> Option<Self> {
        // return
        Self::from_year_ordinal_day(nd.year() as u32, nd.ordinal() as u32)
    }

    /// constructor from year, month, day
    /// returns None on error
    /// # Example
    /// ```
    /// let varweeks = varweeks_millis::VarweekDate::from_ymd(2021,02,28).unwrap();
    /// assert_eq!(varweeks.to_string(),"2021c 09v 3d");
    /// ```
    pub fn from_ymd(year: u32, month: u32, day: u32) -> Option<Self> {
        match chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32) {
            // return
            Some(nd) => Self::from_naive_date(nd),
            None => None,
        }
    }

    /// constructor from str in VarweekDate format
    /// returns None on error
    /// # Example
    /// ```
    /// let varweeks = varweeks_millis::VarweekDate::from_str("2021c 09v 3d").unwrap();
    /// assert_eq!(varweeks.to_string(),"2021c 09v 3d");
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        // ok() Converts from Result<T, E> to Option<T>.
        if s.len() != 12 {
            return None;
        };
        let year = s.get(0..4)?.parse::<u32>().ok()?;
        let year_unit = s.get(4..6)?;
        if year_unit != "c " {
            return None;
        };
        let varweek = s.get(6..8)?.parse::<u32>().ok()?;
        let varweek_unit = s.get(8..10)?;
        if varweek_unit != "v " {
            return None;
        };
        let day = s.get(10..11)?.parse::<u32>().ok()?;
        let day_unit = s.get(11..12)?;
        if day_unit != "d" {
            return None;
        };
        // return
        Self::from_yvd(year, varweek, day)
    }

    /// getter year u32
    /// # Example
    /// ```
    /// let varweeks = varweeks_millis::VarweekDate::from_str("2021c 09v 3d").unwrap();
    /// assert_eq!(varweeks.year(),2021);
    /// ```
    pub fn year(&self) -> u32 {
        // return
        self.year
    }
    /// getter varweek u32
    /// # Example
    /// ```
    /// let varweeks = varweeks_millis::VarweekDate::from_str("2021c 09v 3d").unwrap();
    /// assert_eq!(varweeks.varweek(),9);
    /// ```
    pub fn varweek(&self) -> u32 {
        // return
        self.varweek
    }

    /// getter day u32
    /// # Example
    /// ```
    /// let varweeks = varweeks_millis::VarweekDate::from_str("2021c 09v 3d").unwrap();
    /// assert_eq!(varweeks.day(),3);
    /// ```
    pub fn day(&self) -> u32 {
        // return
        self.day
    }

    /// convert varweek_date to chrono NaiveDate
    /// returns None on error
    /// # Example
    ///
    /// ```
    /// let vd = varweeks_millis::VarweekDate::from_ymd(2021,02,28).unwrap();
    /// let nd = vd.to_naive_date().unwrap();
    /// assert_eq!(nd, chrono::NaiveDate::from_ymd(2021,02,28));
    /// ```
    pub fn to_naive_date(&self) -> Option<chrono::NaiveDate> {
        // return
        chrono::NaiveDate::from_yo_opt(self.year as i32, ((self.varweek - 1) * 7 + self.day) as u32)
    }
}

/// the Trait Display implements fn to_string() implicitly
impl fmt::Display for VarweekDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // return
        write!(
            f,
            r#"{:04}c {:02}v {:01}d"#,
            self.year, self.varweek, self.day
        )
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    // use unwrap::unwrap;

    #[test]
    pub fn t01_naive_date_to_varweek_date() {
        let nd = NaiveDate::from_ymd_opt(2021, 02, 28).unwrap();
        let varweeks = super::VarweekDate::from_naive_date(nd).unwrap();
        assert_eq!(varweeks.to_string(), "2021c 09v 3d");
    }
    #[test]
    pub fn t02_naive_date_to_varweek_date() {
        let nd = NaiveDate::from_ymd_opt(2021, 05, 01).unwrap();
        let varweeks = super::VarweekDate::from_naive_date(nd).unwrap();
        assert_eq!(varweeks.to_string(), "2021c 18v 2d");
    }
    #[test]
    pub fn t03_naive_date_to_varweek_date() {
        let nd = NaiveDate::from_ymd_opt(2021, 12, 25).unwrap();
        let varweeks = super::VarweekDate::from_naive_date(nd).unwrap();
        assert_eq!(varweeks.to_string(), "2021c 52v 2d");
    }
    #[test]
    pub fn t04_varweek_to_naive_date_opt() {
        let varweek = super::VarweekDate::from_str("2021c 09v 3d").unwrap();
        let nd = varweek.to_naive_date().unwrap();
        assert_eq!(nd, NaiveDate::from_ymd_opt(2021, 02, 28).unwrap());
    }
    #[test]
    pub fn t05_varweek_to_naive_date_opt() {
        let varweek = super::VarweekDate::from_str("2021c 52v 2d").unwrap();
        let nd = varweek.to_naive_date().unwrap();
        assert_eq!(nd, NaiveDate::from_ymd_opt(2021, 12, 25).unwrap());
    }
    #[test]
    #[should_panic]
    pub fn t06_varweek_to_naive_date_opt_should_panic() {
        // panic because the varweek format is wrong
        let _varweek = super::VarweekDate::from_str("2021c-09v-3d").unwrap();
    }
}
