use crate::TokenTree;

pub fn add_days_of_year(tokens: &[TokenTree]) -> Vec<String> {
    let mut days = Vec::new();

    match tokens {
        [TokenTree::Literal(lit)] => {
            let year = format!("Y{}", lit.to_string().parse::<u32>().unwrap());
            let fold = std::ffi::OsStr::new(year.as_str());

            for day in std::fs::read_dir(format!("{}{}", crate::BASE_DIR, fold.to_str().unwrap())).unwrap() {
                let day = day.unwrap().file_name();
                let day = day.to_string_lossy();

                if day.starts_with('D') {
                    days.push(format!("pub mod {};", &day[..3]));
                }
            }
        }
        _ => panic!("Invalid argument: expected year 2015 - 2023"),
    }

    days
}

pub fn add_years() -> Vec<String> {
    let mut modules = Vec::new();

    for file in std::fs::read_dir(crate::BASE_DIR).unwrap() {
        let file = file.unwrap().file_name();
        let file = file.to_string_lossy();

        if file.starts_with('Y') {
            modules.push(format!("mod {};", file));
        }
    }

    modules
}
