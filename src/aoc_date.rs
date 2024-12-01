use chrono::Datelike;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Day(u8);

impl Day {
    pub fn new(day: u8) -> Option<Self> {
        if day == 0 || day > 25 {
            return None;
        }
        Some(Self(day))
    }

    // Not part of the public API
    #[doc(hidden)]
    pub const fn __new_unchecked(day: u8) -> Self {
        Self(day)
    }

    pub fn into_inner(self) -> u8 {
        self.0
    }
}

impl std::str::FromStr for Day {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day = s.parse().map_err(|_| "Failed to parse day")?;
        if day < 1 || day > 25 {
            return Err("Day must be between 1 and 25");
        }
        Ok(Day(day))
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AocDate {
    pub year: u16,
    pub day: Day,
}

impl AocDate {
    pub fn new(day: Option<Day>, year: Option<u16>) -> Self {

        let date = get_advent_of_code_date();
        Self {
            year: year
                .or(std::env::var("AOC_YEAR").ok().and_then(|x| x.parse().ok()))
                .or(get_current_year(&date)).unwrap(),
            day: day.unwrap_or(get_current_day(&date).unwrap()),
        }
    }
}

// Get the current date in the EST timezone, which is used by advent of code (-05:00)
fn get_advent_of_code_date() -> chrono::NaiveDate {
    const EST_OFFSET: chrono::FixedOffset = chrono::FixedOffset::east_opt(-5 * 60 * 60).unwrap();
    let utc_now = chrono::Utc::now();
    let est_now = utc_now.with_timezone(&EST_OFFSET);
    est_now.date_naive()
}


fn get_current_year(date: &chrono::NaiveDate) -> Option<u16> {
    (date.year() as u16).checked_sub(2015)
}

fn get_current_day(date: &chrono::NaiveDate) -> Option<Day> {
    Day::new(date.day() as u8)
}

// Creates Day value in a const context.
#[macro_export]
macro_rules! day {
    ($day:expr) => {{
        const _ASSERT: () = assert!(
            $day != 0 && $day <= 25,
            concat!(
                "invalid day number `",
                $day,
                "`, expecting a value between 1 and 25"
            ),
        );
        $crate::aoc_date::Day::__new_unchecked($day)
    }};
}

// Creates AocDate value in a const context.
#[macro_export]
macro_rules! aoc_date {
    ($day:expr, $year:expr) => {

        const day = day!($day);

        const _ASSERT: () = assert!(
            $year >= 2015,
            concat!(
                "invalid year `",
                $year,
                "`, expecting a value greater than or equal to 2015"
            ),
        );

        $crate::aoc_date::AocDate::new(Some(day), Some($year))
    };
}