use gigasecond;

use chrono::{TimeZone, Utc};

#[test]
fn test_case_1() {
    let start_date = Utc.ymd(1996, 5, 5).and_hms(0, 0, 0);

    assert_eq!(
        gigasecond::after(start_date),
        Utc.ymd(2028, 1, 12).and_hms(1, 46, 40)
    );
}

#[test]
fn test_case_2() {
    let start_date = Utc.ymd(1945, 9, 2).and_hms(0, 0, 0);

    assert_eq!(
        gigasecond::after(start_date),
        Utc.ymd(1977, 5, 11).and_hms(1, 46, 40)
    );
}

#[test]
fn test_case_3() {
    let start_date = Utc.ymd(2022, 1, 5).and_hms(0, 0, 0);

    assert_eq!(
        gigasecond::after(start_date),
        Utc.ymd(2053, 9, 13).and_hms(1, 46, 40)
    );
}

#[test]
fn test_case_4() {
    let start_date = Utc.ymd(2019, 10, 12).and_hms(15, 20, 43);

    assert_eq!(
        gigasecond::after(start_date),
        Utc.ymd(2051, 06, 20).and_hms(17, 07, 23)
    );
}

#[test]
fn test_case_5() {
    let start_date = Utc.ymd(1000, 11, 11).and_hms(23, 59, 59);

    assert_eq!(
        gigasecond::after(start_date),
        Utc.ymd(1032, 07, 21).and_hms(1, 46, 39)
    );
}