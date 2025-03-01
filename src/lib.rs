use chrono::NaiveDate;

const EPOCH_BEFORE_MID_MARCH: u16 = 1469;
const EPOCH_ON_OR_AFTER_MID_MARCH: u16 = 1468;
const DAYS_IN_MONTHS: [i32; 12] = [31, 31, 31, 31, 31, 30, 30, 30, 30, 30, 30, 30];
const MONTH_NAMES: [&'static str; 12] = [
    "Chet", "Vaisakh", "Jeth", "Harh", "Sawan", "Bhadon", "Assu", "Kattak", "Maghar", "Poh",
    "Magh", "Phaggan",
];

pub struct NanakshahiDate {
    pub year: u16,
    pub month: &'static str,
    pub day: u8,
}

/// Convert a Gregorian date to a Nanakshahi date.
///
/// # Examples
/// ```
/// let year = 1970;
/// let month = 1;
/// let day = 1;
///
/// let date = nanakshahi::to(year, month, day);
/// ```
pub fn to(year: u16, month: u8, day: u8) -> NanakshahiDate {
    let epoch: u16 = if month > 3 || (month == 3 && day >= 14) {
        EPOCH_ON_OR_AFTER_MID_MARCH
    } else {
        EPOCH_BEFORE_MID_MARCH
    };
    let mut offset: i64 = days_between(year, month, day);

    for (index, &days) in DAYS_IN_MONTHS.iter().enumerate() {
        if offset < days as i64 {
            return NanakshahiDate {
                year: year - epoch,
                month: MONTH_NAMES[index],
                day: (offset + 1) as u8,
            };
        } else {
            offset -= days as i64;
        }
    }

    // If we fall through the loop (which should not happen), panic.
    panic!("Offset exceeded the total number of days in the Nanakshahi year");
}

fn days_between(year: u16, month: u8, day: u8) -> i64 {
    let offset: u16 = if month > 3 || (month == 3 && day >= 14) {
        0
    } else {
        1
    };
    let date: NaiveDate =
        NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).expect("Invalid date");
    let reference_date: NaiveDate =
        NaiveDate::from_ymd_opt((year - offset) as i32, 3, 14).expect("Invalid date");
    (date - reference_date).num_days()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_on_mid_march() {
        let date: NanakshahiDate = to(2025, 3, 14);

        assert_eq!(date.year, 557);
        assert_eq!(date.month, "Chet");
        assert_eq!(date.day, 1);
    }

    #[test]
    fn test_to_before_mid_march() {
        let date: NanakshahiDate = to(2025, 3, 13);

        assert_eq!(date.year, 556);
        assert_eq!(date.month, "Phaggan");
        assert_eq!(date.day, 30);
    }
}
