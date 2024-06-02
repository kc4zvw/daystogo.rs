
/// Return whether year `y` in the Gregorian calendar is a leap year

fn is_leap_year(y: u32) -> bool
{
	y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

/// Return the number of days in the month `m` in year `y` in the Gregorian calendar. Note that
/// The month number is zero based, i.e. `m=0` corresponds to January, `m=1` to February, etc.

fn days_per_month(y: u32, m: u32) -> u32
{
    const DAYS_PER_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let nd = DAYS_PER_MONTH[m as usize];
    nd + (m == 1 && is_leap_year(y)) as u32
}


fn datediff<T>(date0: T, date1: T) -> (u32, u32, u32, bool)
where T: chrono::Datelike + PartialOrd {

    if date1 < date0 {
        let (ny, nm, nd, _) = datediff(date1, date0);
        (ny, nm, nd, true)
    } else {
        let (y0, m0, mut d0) = (date0.year() as u32, date0.month0(), date0.day0());
        let (mut y1, mut m1, mut d1) = (date1.year() as u32, date1.month0(), date1.day0());

        if d0 > d1 {
            let (py1, pm1) = if m1 == 0 { (y1-1, 11) } else { (y1, m1-1) };
            let pnd = days_per_month(py1, pm1);
            d0 = d0.min(pnd-1);
            if d0 > d1 {
                y1 = py1;
                m1 = pm1;
                d1 += pnd;
            }
        }
        if m0 > m1 {
            y1 -= 1;
            m1 += 12;
        }

        (y1 - y0, m1 - m0, d1 - d0, false)
    }
}

