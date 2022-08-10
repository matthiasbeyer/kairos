#[inline]
pub fn adjust_times_add(
    mut y: i64,
    mut mo: i64,
    mut d: i64,
    mut h: i64,
    mut mi: i64,
    mut s: i64,
) -> (i64, i64, i64, i64, i64, i64) {
    // Subtract $border from the $base as long as the $base is bigger or equal to the $border.
    // The number of subtractions are added to $next.
    macro_rules! fix {
        {
            $base:ident,
            $border:expr,
            $next:ident
        } => {
            $next += ($base - ($base % $border)) / $border;
            $base = $base % $border;
        }
    }

    fix! { s , 60, mi }
    fix! { mi, 60, h  }
    fix! { h , 24, d  }

    while d > get_num_of_days_in_month(y, mo) {
        d -= get_num_of_days_in_month(y, mo);
        mo += 1;
    }

    while mo > 12 {
        y += 1;
        mo -= 12;
    }

    (y, mo, d, h, mi, s)
}

#[inline]
pub fn adjust_times_sub(
    mut y: i64,
    mut mo: i64,
    mut d: i64,
    mut h: i64,
    mut mi: i64,
    mut s: i64,
) -> (i64, i64, i64, i64, i64, i64) {
    if s < 0 {
        mi -= (s.abs() / 60) + 1;
        s = 60 - (0 - s).abs();
    }

    if mi < 0 {
        h -= (mi.abs() / 60) + 1;
        mi = 60 - (0 - mi).abs();
    }

    if h < 0 {
        d -= (h.abs() / 24) + 1;
        h = 24 - (0 - h).abs();
    }

    if d < 1 {
        mo -= (d.abs() / 32) + 1;
        d = 31 - (0 - d).abs();
    }

    if mo < 1 {
        y -= (mo.abs() / 13) + 1;
        mo = 12 - (0 - mo).abs();
    }

    (y, mo, d, h, mi, s)
}

#[inline]
pub fn get_num_of_days_in_month(y: i64, m: i64) -> i64 {
    if m == 1 || m == 3 || m == 5 || m == 7 || m == 8 || m == 10 || m == 12 {
        31
    } else if m == 2 {
        if is_leap_year(y as i32) {
            29
        } else {
            28
        }
    } else {
        30
    }
}

#[inline]
pub fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0) && (y % 100 != 0 || y % 400 == 0)
}

#[test]
fn test_is_leap_year() {
    let leaps = [
        1880, 1884, 1888, 1892, 1896, 1904, 1908, 1912, 1916, 1920, 1924, 1928, 1932, 1936, 1940,
        1944, 1948, 1952, 1956, 1960, 1964, 1968, 1972, 1976, 1980, 1984, 1988, 1992, 1996, 2000,
        2004, 2008, 2012, 2020, 2024, 2028, 2032, 2036, 2040, 2044, 2048, 2052, 2056, 2060, 2064,
        2068, 2072, 2076, 2080, 2084, 2088, 2092, 2096, 2104, 2108, 2112, 2116, 2120, 2124, 2128,
        2132, 2136, 2140, 2144, 2148, 2152, 2156, 2160, 2164, 2168, 2172, 2176, 2180, 2184, 2188,
        2192, 2196, 2204, 2208,
    ];

    for i in leaps.iter() {
        assert!(is_leap_year(*i), "Is no leap year: {}", i);
        assert!(
            !is_leap_year(*i + 1),
            "Seems to be leap year: {}, but should not be",
            i + 1
        );
    }
}

#[test]
fn test_get_num_of_days_in_month() {
    assert_eq!(31, get_num_of_days_in_month(2000, 01));
    assert_eq!(29, get_num_of_days_in_month(2000, 02));
    assert_eq!(31, get_num_of_days_in_month(2000, 03));
    assert_eq!(30, get_num_of_days_in_month(2000, 04));
    assert_eq!(31, get_num_of_days_in_month(2000, 05));
    assert_eq!(30, get_num_of_days_in_month(2000, 06));
    assert_eq!(31, get_num_of_days_in_month(2000, 07));
    assert_eq!(31, get_num_of_days_in_month(2000, 08));
    assert_eq!(30, get_num_of_days_in_month(2000, 09));
    assert_eq!(31, get_num_of_days_in_month(2000, 10));
    assert_eq!(30, get_num_of_days_in_month(2000, 11));
    assert_eq!(31, get_num_of_days_in_month(2000, 12));

    assert_eq!(31, get_num_of_days_in_month(2001, 01));
    assert_eq!(28, get_num_of_days_in_month(2001, 02));
    assert_eq!(31, get_num_of_days_in_month(2001, 03));
    assert_eq!(30, get_num_of_days_in_month(2001, 04));
    assert_eq!(31, get_num_of_days_in_month(2001, 05));
    assert_eq!(30, get_num_of_days_in_month(2001, 06));
    assert_eq!(31, get_num_of_days_in_month(2001, 07));
    assert_eq!(31, get_num_of_days_in_month(2001, 08));
    assert_eq!(30, get_num_of_days_in_month(2001, 09));
    assert_eq!(31, get_num_of_days_in_month(2001, 10));
    assert_eq!(30, get_num_of_days_in_month(2001, 11));
    assert_eq!(31, get_num_of_days_in_month(2001, 12));

    assert_eq!(31, get_num_of_days_in_month(2002, 01));
    assert_eq!(28, get_num_of_days_in_month(2002, 02));
    assert_eq!(31, get_num_of_days_in_month(2002, 03));
    assert_eq!(30, get_num_of_days_in_month(2002, 04));
    assert_eq!(31, get_num_of_days_in_month(2002, 05));
    assert_eq!(30, get_num_of_days_in_month(2002, 06));
    assert_eq!(31, get_num_of_days_in_month(2002, 07));
    assert_eq!(31, get_num_of_days_in_month(2002, 08));
    assert_eq!(30, get_num_of_days_in_month(2002, 09));
    assert_eq!(31, get_num_of_days_in_month(2002, 10));
    assert_eq!(30, get_num_of_days_in_month(2002, 11));
    assert_eq!(31, get_num_of_days_in_month(2002, 12));

    assert_eq!(31, get_num_of_days_in_month(2003, 01));
    assert_eq!(28, get_num_of_days_in_month(2003, 02));
    assert_eq!(31, get_num_of_days_in_month(2003, 03));
    assert_eq!(30, get_num_of_days_in_month(2003, 04));
    assert_eq!(31, get_num_of_days_in_month(2003, 05));
    assert_eq!(30, get_num_of_days_in_month(2003, 06));
    assert_eq!(31, get_num_of_days_in_month(2003, 07));
    assert_eq!(31, get_num_of_days_in_month(2003, 08));
    assert_eq!(30, get_num_of_days_in_month(2003, 09));
    assert_eq!(31, get_num_of_days_in_month(2003, 10));
    assert_eq!(30, get_num_of_days_in_month(2003, 11));
    assert_eq!(31, get_num_of_days_in_month(2003, 12));

    assert_eq!(31, get_num_of_days_in_month(2004, 01));
    assert_eq!(29, get_num_of_days_in_month(2004, 02));
    assert_eq!(31, get_num_of_days_in_month(2004, 03));
    assert_eq!(30, get_num_of_days_in_month(2004, 04));
    assert_eq!(31, get_num_of_days_in_month(2004, 05));
    assert_eq!(30, get_num_of_days_in_month(2004, 06));
    assert_eq!(31, get_num_of_days_in_month(2004, 07));
    assert_eq!(31, get_num_of_days_in_month(2004, 08));
    assert_eq!(30, get_num_of_days_in_month(2004, 09));
    assert_eq!(31, get_num_of_days_in_month(2004, 10));
    assert_eq!(30, get_num_of_days_in_month(2004, 11));
    assert_eq!(31, get_num_of_days_in_month(2004, 12));
}
