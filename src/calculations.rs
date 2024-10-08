use chrono::{DateTime, Datelike, Days, Local, NaiveDate};

fn days_between_dates(start_date: NaiveDate, end_date: NaiveDate) -> u32 {
    return (end_date.num_days_from_ce() - start_date.num_days_from_ce()) as u32;
}

pub fn last_day_of_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
        .pred_opt()
        .unwrap()
        .day()
}

// Abyss resets every 16th day of the month
// Time until next rotation =  16 - today or time until last day of
// the month + 16.
pub fn get_abyss_rotation_count(mut days: u32) -> u32 {
    let mut rotation_count: u32 = 0;
    let mut today: DateTime<Local> = Local::now();
    let mut time_until_next_rotation: u32 = if today.day() < 16 {
        16 - today.day()
    } else {
        last_day_of_month(today.year(), today.month()) - today.day() + 16
    };
    while days >= time_until_next_rotation {
        match today.checked_add_days(Days::new(time_until_next_rotation as u64)) {
            Some(new_date) => {
                today = new_date;
                rotation_count += 1;
                days -= time_until_next_rotation
            }
            None => {
                println!("There was an unexpected error when advancing dates (Abyss)");
                break;
            }
        }
        if today.day() < 16 {
            time_until_next_rotation = 16 - today.day();
        } else {
            time_until_next_rotation =
                last_day_of_month(today.year(), today.month()) - today.day() + 16;
        }
    }
    return rotation_count;
}

// Imaginarium theater resets every 1st day of the month
// Time until next reset = last_day_of_the_month + 1
// Max imaginarium stages: 10
pub fn get_imaginarium_theater_rotation_count(mut days: u32) -> u32 {
    let mut rotation_count: u32 = 0;
    let mut today: DateTime<Local> = Local::now();
    let mut time_until_next_rotation =
        last_day_of_month(today.year(), today.month()) - today.day() + 1;

    while days >= time_until_next_rotation {
        match today.checked_add_days(Days::new(time_until_next_rotation as u64)) {
            Some(new_date) => {
                today = new_date;
                rotation_count += 1;
                days -= time_until_next_rotation;
            }
            None => {
                println!("There was an error when trying to advance dates (Imaginarium Theater)")
            }
        }
        time_until_next_rotation = last_day_of_month(today.year(), today.month()) - today.day() + 1;
    }
    return rotation_count;
}

// Shop resets every 1st day of the month.
// Time until next reset = time until the last day of the month + 1.
pub fn get_shop_reset_count(mut days: u32) -> u32 {
    let mut reset_count: u32 = 0;
    let mut today: DateTime<Local> = Local::now();
    let mut time_until_next_reset: u32 =
        last_day_of_month(today.year(), today.month()) - today.day() + 1;
    while days >= time_until_next_reset {
        match today.checked_add_days(Days::new(time_until_next_reset as u64)) {
            Some(new_date) => {
                today = new_date;
                reset_count += 1;
                days -= time_until_next_reset;
            }
            None => println!("There was an unexpected error when advancing dates (shop reset)"),
        }
        time_until_next_reset = last_day_of_month(today.year(), today.month()) - today.day() + 1;
    }
    return reset_count;
}

pub fn estimate_primogems(
    days: u32,
    blessing: bool,
    three_star_chambers: u32,
    imaginarium_stages: u32,
) -> String {
    // total estimation.
    let mut primogem_estimation: u32 = 0;
    // All types of primogems.
    let daily_primogems: u32 = 60 * days;
    let mut blessing_primogems: u32 = 0;
    let mut abyss_primogems: u32 = 0;
    let mut imaginarium_primogems: u32 = 0;

    // handle blessing.
    if blessing {
        blessing_primogems = 90 * days;
    }

    // handle abyss.
    if three_star_chambers > 0 {
        // Every 3 chambers gives a 50 primogem bonus, thus line 123
        abyss_primogems = get_abyss_rotation_count(days)
            * ((50 * three_star_chambers) + ((three_star_chambers / 3) * 50));
    }

    //handle Imaginarium Theater.
    if imaginarium_stages > 0 {
        // All stages give 60 primos except 3, 6 , 8, 10.
        let imaginarium_rotations = get_imaginarium_theater_rotation_count(days);
        imaginarium_primogems = imaginarium_rotations * 60 * imaginarium_stages;

        // handle bonuses
        match imaginarium_stages {
            3 => imaginarium_primogems += imaginarium_rotations * 40,
            6 => imaginarium_primogems += imaginarium_rotations * 80,
            8 => imaginarium_primogems += imaginarium_rotations * 140,
            10 => imaginarium_primogems += imaginarium_rotations * 200,
            _ => (),
        }
    }

    // handle shop resets.
    let shop_wishes = get_shop_reset_count(days) * 5;

    // total_primogems.
    primogem_estimation +=
        daily_primogems + blessing_primogems + abyss_primogems + imaginarium_primogems;

    // Done in multiple push_str due to format and readability. Putting the whole string in a single format!() somehow
    // messes up the UI format.
    let mut detailed_wish_estimation = String::new();
    detailed_wish_estimation.push_str(
        format!(
            "- Daily primogems: {daily_primogems}\n
        - Abyss primogems: {abyss_primogems}\n
        - Imaginarium Theater primogems: {imaginarium_primogems}\n
        - Blessing primogems: {blessing_primogems}\n
        - Shop wishes: {shop_wishes}\n
        __***Estimated total primogems: {primogem_estimation}***__\n
        __***Estimated total wishes: {} and {} primogems***__",
            (primogem_estimation as f64 / 160.0).floor(),
            primogem_estimation % 160
        )
        .as_str(),
    );

    return detailed_wish_estimation;
}
pub fn estimation_between_dates(
    start_date: NaiveDate,
    end_date: NaiveDate,
    blessing: bool,
    three_star_chambers: u32,
    imaginarium_stages: u32,
) -> String {
    let days = days_between_dates(start_date, end_date);
    return estimate_primogems(days, blessing, three_star_chambers, imaginarium_stages);
}

pub fn estimation_from_today_until(
    end_date: NaiveDate,
    blessing: bool,
    three_star_chambers: u32,
    imaginarium_stages: u32,
) -> String {
    let today: NaiveDate = Local::now().date_naive();
    let days = days_between_dates(today, end_date);
    return estimate_primogems(days, blessing, three_star_chambers, imaginarium_stages);
}
