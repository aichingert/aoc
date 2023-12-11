pub fn add_fns_of_year(year: u64) -> String {
    let mut fns = vec![String::from("no_solution"); 25];

    let year = format!("Y{}", year);
    let fold = std::ffi::OsStr::new(year.as_str());

    if let Ok(directory) = std::fs::read_dir(format!("{}{}", crate::BASE_DIR, fold.to_str().unwrap())) {
        for day in directory {
            let day = day.unwrap().file_name();

            if day.to_string_lossy().starts_with("D") {
                let ident: &str = &day.to_string_lossy()[..3];
                let ptr: usize = &ident[1..].parse::<usize>().unwrap() - 1;

                fns[ptr] = format!("{year}::{ident}::solve");
            }
        }
    }
    
    let mut fns = fns.join(",");
    fns.insert(0, '[');
    fns.push(']');

    fns
}
