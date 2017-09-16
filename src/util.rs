#[inline]
pub fn adjust_times_add(mut y: i64, mut mo: i64, mut d: i64, mut h: i64, mut mi: i64, mut s: i64)
    -> (i64, i64, i64, i64, i64, i64)
{
    macro_rules! fix {
        {
            $base:ident,
            $border:expr,
            $next:ident
        } => {
            while $base >= $border {
                $next += 1;
                $base -= $border;
            }
        }
    }

    fix! { s , 60, mi }
    fix! { mi, 60, h  }
    fix! { h , 24, d  }

    if mo == 1 || mo == 3 || mo == 5 || mo == 7 || mo == 8 || mo == 10 || mo == 12 {
        fix! { d , 31, mo }
    } else {
        fix! { d , 30, mo }
    }

    fix! { mo, 12, y  }

    (y, mo, d, h, mi, s)
}

#[inline]
pub fn adjust_times_sub(mut y: i64, mut mo: i64, mut d: i64, mut h: i64, mut mi: i64, mut s: i64)
    -> (i64, i64, i64, i64, i64, i64)
{

    println!("s < 0  -> = {}", s);
    if s < 0 {
        println!("mi -= {}", (s.abs() / 60) + 1);
        println!("s   = {}", (60 - (0 - s).abs()));

        mi -= (s.abs() / 60) + 1;
        s   = 60 - (0 - s).abs();
    }
    println!("");

    println!("mi < 0  -> = {}", mi);
    if mi < 0 {
        println!("h -= {}", (mi.abs() / 60) + 1);
        println!("mi = {}", (60 - (0 - mi).abs()));

        h -= (mi.abs() / 60) + 1;
        mi = 60 - (0 - mi).abs();
    }
    println!("");

    println!("h < 0  -> = {}", h);
    if h < 0 {
        println!("d -= {}", (h.abs() / 24) + 1);
        println!("h  = {}", (24 - (0 - h).abs()));

        d -= (h.abs() / 24) + 1;
        h  = 24 - (0 - h).abs();
    }
    println!("");

    println!("d < 1  -> = {}", d);
    if d < 1 {
        println!("mo -= {}", (d.abs() / 32) + 1);
        println!("d   = {}", (31 - (0 - d).abs()));

        mo -= (d.abs() / 32) + 1;
        d   = 31 - (0 - d).abs();
    }
    println!("");

    println!("mo < 1  -> = {}", mo);
    if mo < 1 {
        println!("y -= {}", (mo.abs() / 13) + 1);
        println!("mo = {}", (12 - (0 - mo).abs()));

        y -= (mo.abs() / 13) + 1;
        mo = 12 - (0 - mo).abs();
    }

    (y, mo, d, h, mi, s)
}


#[inline]
pub fn get_num_of_days_in_month(y: i32, m: u32) -> u32 {
    if m == 1 || m == 3 || m == 5 || m == 7 || m == 8 || m == 10 || m == 12 {
        31
    } else if m == 2 {
        if is_leap_year(y) {
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
    let leaps = [ 1880, 1884, 1888, 1892, 1896, 1904, 1908, 1912, 1916, 1920, 1924, 1928, 1932,
        1936, 1940, 1944, 1948, 1952, 1956, 1960, 1964, 1968, 1972, 1976, 1980, 1984, 1988, 1992,
        1996, 2000, 2004, 2008, 2012, 2020, 2024, 2028, 2032, 2036, 2040, 2044, 2048, 2052, 2056,
        2060, 2064, 2068, 2072, 2076, 2080, 2084, 2088, 2092, 2096, 2104, 2108, 2112, 2116, 2120,
        2124, 2128, 2132, 2136, 2140, 2144, 2148, 2152, 2156, 2160, 2164, 2168, 2172, 2176, 2180,
        2184, 2188, 2192, 2196, 2204, 2208 ];

    for i in leaps.iter() {
        assert!(is_leap_year(*i), "Is no leap year: {}", i);
        assert!(!is_leap_year(*i + 1), "Seems to be leap year: {}, but should not be", i + 1);
    }

}
