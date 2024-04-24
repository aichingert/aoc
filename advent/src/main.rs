advent_macros::add_years!();
advent_macros::add_fn_pointers!();

fn main() {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() == 1 && &args[0] == "help" {
        println!("Usage: cargo r [y(Year)] [d(Day)]");
        return;
    }

    let (mut year, mut day): (Option<usize>, Option<usize>) = (None, None);

    for arg in args.iter_mut() {
        if arg.strip_prefix('y').is_some() {
            let n = arg.parse::<usize>().unwrap();

            if (2015..=CUR_YEAR).contains(&n) {
                year = Some(n - 2015);
            } else {
                println!("Invalid year: (2015-{}", CUR_YEAR);
                return;
            }
        } else if arg.strip_prefix('d').is_some() {
            let n = arg[1..].parse::<usize>().unwrap();

            if (1..=25).contains(&n) {
                day = Some(n - 1);
            } else {
                println!("Invalid day: (1-25)");
                return
            }
        }
    }

    match (year, day) {
        (Some(year), Some(day)) => {
            println!("Year {}, Day {}", year + 2015, day + 1);
            FN_POINTER[year][day]();
        }
        (Some(year), _) => {
            println!("Year {}", year + 2015);
            println!("===========");
            for day in &FN_POINTER[year] {
                day();
            }
        }
        (_, Some(day)) => {
            println!("Day {}s", day + 1);
            println!("=========");

            for year in &FN_POINTER {
                year[day]();
            }
        }
        (_, _) => LATEST(),
    }

}
