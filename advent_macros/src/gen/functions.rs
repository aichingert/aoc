use std::time::SystemTime;
use std::fs;

pub fn add_fns_of_year(year: u64) -> String {
    let mut fns = vec![String::from("no_solution"); 25];

    let year = format!("Y{}", year);
    let fold = std::ffi::OsStr::new(year.as_str());

    if let Ok(directory) = fs::read_dir(format!("{}{}", crate::BASE_DIR, fold.to_str().unwrap())) {
        for day in directory {
            let day = day.unwrap().file_name();

            if day.to_string_lossy().starts_with("D") {
                let ident: &str = &day.to_string_lossy()[..3];
                let ptr: usize = &ident[1..].parse::<usize>().unwrap() - 1;

                fns[ptr] = format!("{year}::{ident}::{}", crate::SOLUTION);
            }
        }
    }
    
    let mut fns = fns.join(",");
    fns.insert(0, '[');
    fns.push(']');

    fns
}

pub fn find_last_edited() -> String {
    let mut time = SystemTime::UNIX_EPOCH;
    let mut module: Option<String> = None;

    for dir in fs::read_dir(crate::BASE_DIR).unwrap() {
        let dir = dir.unwrap().path();

        if dir.is_dir() {
            for file in fs::read_dir(&dir).unwrap() {
                let file = file.unwrap();
                let path = file.path();
                let name = file.file_name().as_os_str().to_string_lossy().to_string();

                let last_edited = std::fs::metadata(&path).unwrap().modified().unwrap();

                if name.starts_with("D") && last_edited > time {
                    time = last_edited;

                    let year = format!("{}", dir.display());
                    let year = year.split('/').collect::<Vec<_>>()[2];

                    module = Some(format!("{}::{}", year, &name[..3]));
                }
            }
        }
    }

    if let Some(module) = module {
        format!("const LATEST: fn() -> () = {}::{};", module, crate::SOLUTION)
    } else {
        String::from("const LATEST: fn() -> () = no_solution;")
    }
}
