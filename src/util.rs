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
            $base %= $border;
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
        1880, 1884, 1888, 1892, 1896, 1904, 1908, 1912, 1916, 1920, 1924, 1928, 1932, 1936, 1940, 1944, 1948, 1952,
        1956, 1960, 1964, 1968, 1972, 1976, 1980, 1984, 1988, 1992, 1996, 2000, 2004, 2008, 2012, 2020, 2024, 2028,
        2032, 2036, 2040, 2044, 2048, 2052, 2056, 2060, 2064, 2068, 2072, 2076, 2080, 2084, 2088, 2092, 2096, 2104,
        2108, 2112, 2116, 2120, 2124, 2128, 2132, 2136, 2140, 2144, 2148, 2152, 2156, 2160, 2164, 2168, 2172, 2176,
        2180, 2184, 2188, 2192, 2196, 2204, 2208,
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
    assert_eq!(31, get_num_of_days_in_month(2000, 1));
    assert_eq!(29, get_num_of_days_in_month(2000, 2));
    assert_eq!(31, get_num_of_days_in_month(2000, 3));
    assert_eq!(30, get_num_of_days_in_month(2000, 4));
    assert_eq!(31, get_num_of_days_in_month(2000, 5));
    assert_eq!(30, get_num_of_days_in_month(2000, 6));
    assert_eq!(31, get_num_of_days_in_month(2000, 7));
    assert_eq!(31, get_num_of_days_in_month(2000, 8));
    assert_eq!(30, get_num_of_days_in_month(2000, 9));
    assert_eq!(31, get_num_of_days_in_month(2000, 10));
    assert_eq!(30, get_num_of_days_in_month(2000, 11));
    assert_eq!(31, get_num_of_days_in_month(2000, 12));

    assert_eq!(31, get_num_of_days_in_month(2001, 1));
    assert_eq!(28, get_num_of_days_in_month(2001, 2));
    assert_eq!(31, get_num_of_days_in_month(2001, 3));
    assert_eq!(30, get_num_of_days_in_month(2001, 4));
    assert_eq!(31, get_num_of_days_in_month(2001, 5));
    assert_eq!(30, get_num_of_days_in_month(2001, 6));
    assert_eq!(31, get_num_of_days_in_month(2001, 7));
    assert_eq!(31, get_num_of_days_in_month(2001, 8));
    assert_eq!(30, get_num_of_days_in_month(2001, 9));
    assert_eq!(31, get_num_of_days_in_month(2001, 10));
    assert_eq!(30, get_num_of_days_in_month(2001, 11));
    assert_eq!(31, get_num_of_days_in_month(2001, 12));

    assert_eq!(31, get_num_of_days_in_month(2002, 1));
    assert_eq!(28, get_num_of_days_in_month(2002, 2));
    assert_eq!(31, get_num_of_days_in_month(2002, 3));
    assert_eq!(30, get_num_of_days_in_month(2002, 4));
    assert_eq!(31, get_num_of_days_in_month(2002, 5));
    assert_eq!(30, get_num_of_days_in_month(2002, 6));
    assert_eq!(31, get_num_of_days_in_month(2002, 7));
    assert_eq!(31, get_num_of_days_in_month(2002, 8));
    assert_eq!(30, get_num_of_days_in_month(2002, 9));
    assert_eq!(31, get_num_of_days_in_month(2002, 10));
    assert_eq!(30, get_num_of_days_in_month(2002, 11));
    assert_eq!(31, get_num_of_days_in_month(2002, 12));

    assert_eq!(31, get_num_of_days_in_month(2003, 1));
    assert_eq!(28, get_num_of_days_in_month(2003, 2));
    assert_eq!(31, get_num_of_days_in_month(2003, 3));
    assert_eq!(30, get_num_of_days_in_month(2003, 4));
    assert_eq!(31, get_num_of_days_in_month(2003, 5));
    assert_eq!(30, get_num_of_days_in_month(2003, 6));
    assert_eq!(31, get_num_of_days_in_month(2003, 7));
    assert_eq!(31, get_num_of_days_in_month(2003, 8));
    assert_eq!(30, get_num_of_days_in_month(2003, 9));
    assert_eq!(31, get_num_of_days_in_month(2003, 10));
    assert_eq!(30, get_num_of_days_in_month(2003, 11));
    assert_eq!(31, get_num_of_days_in_month(2003, 12));

    assert_eq!(31, get_num_of_days_in_month(2004, 1));
    assert_eq!(29, get_num_of_days_in_month(2004, 2));
    assert_eq!(31, get_num_of_days_in_month(2004, 3));
    assert_eq!(30, get_num_of_days_in_month(2004, 4));
    assert_eq!(31, get_num_of_days_in_month(2004, 5));
    assert_eq!(30, get_num_of_days_in_month(2004, 6));
    assert_eq!(31, get_num_of_days_in_month(2004, 7));
    assert_eq!(31, get_num_of_days_in_month(2004, 8));
    assert_eq!(30, get_num_of_days_in_month(2004, 9));
    assert_eq!(31, get_num_of_days_in_month(2004, 10));
    assert_eq!(30, get_num_of_days_in_month(2004, 11));
    assert_eq!(31, get_num_of_days_in_month(2004, 12));
}
