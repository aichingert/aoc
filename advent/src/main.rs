advent_macros::add_years!();
advent_macros::add_fn_pointers!();

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.len() == 1 && &args[0] == "help" {
        println!("Usage: cargo r [y(Year)] [d(Day)]");
        return;
    }

    let (mut year, mut day): (Option<usize>, Option<usize>) = (None, None);

    for i in 0..args.len() {
        if args[i].starts_with("y") {
            let n = args[i][1..].parse::<usize>().unwrap();

            if n >= 2015 && n <= CUR_YEAR {
                year = Some(n - 2015);
            } else {
                println!("Invalid year: (2015-{}", CUR_YEAR);
                return;
            }
        } else if args[i].starts_with("d") {
            let n = args[i][1..].parse::<usize>().unwrap();

            if n >= 1 && n <= 25 {
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
            for day in 0..FN_POINTER[year].len() {
                FN_POINTER[year][day]();
            }
        }
        (_, Some(day)) => {
            println!("Day {}s", day + 1);
            println!("=========");

            for year in 0..FN_POINTER.len() {
                FN_POINTER[year][day]();
            }
        }
        (_, _) => LATEST(),
    }

}
