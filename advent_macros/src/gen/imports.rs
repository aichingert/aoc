use crate::TokenTree;

pub fn get_days(tokens: &[TokenTree]) -> Vec<String> {
    let mut days = Vec::new();

    match tokens {
        [TokenTree::Literal(lit)] => {
            let year = format!("Y{}", lit.to_string().parse::<u32>().unwrap());
            let fold = std::ffi::OsStr::new(year.as_str());

            for path in std::fs::read_dir(crate::BASE_DIR).unwrap() {
                let path = path.unwrap().file_name();

                if path == fold {
                    for day in std::fs::read_dir(format!("{}{}", crate::BASE_DIR, fold.to_str().unwrap())).unwrap() {
                        let day = day.unwrap().file_name();

                        if day.to_string_lossy().starts_with("D") {
                            days.push(format!("pub mod {};", &day.to_string_lossy()[..3]));
                        }
                    }
                }
            }
        }
        _ => panic!("Invalid argument: expected year 2015 - 2023"),
    }

    days
}
