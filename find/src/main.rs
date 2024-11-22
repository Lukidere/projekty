use std::fs::read_dir;
use std::env;
use std::thread;
fn is_path(cos: &str) -> bool {
    if read_dir(cos).is_ok() {
        true
    } else {
        false
    }
}
fn does_contain(dir:&str,searched:&str) -> bool {
        if dir.contains(searched) && !dir.contains("proc") {
            println!("{dir}");
            true
            
        } else {
            false
        }
}
fn read_next<'a>(dir: String,searched:&String ,every:&String ,files:&String) {
    let paths = read_dir(&dir).unwrap();
    for path in paths {
        match path.ok() {
            Some(val) => match val.path().to_str() {
                Some(s) => match is_path(&s) {
                    true => {
                        if does_contain(s,&searched) && every.to_lowercase().contains("f") {
                            break
                        } else {
                            read_next(s.to_owned(),searched,every,&files)
                        }
                            },
                    false => {
                        if files.to_lowercase().contains("t") {
                            if does_contain(s,&searched.clone()) && every.to_lowercase().contains("f") {
                                break
                            }
                        };
                        continue
                    }
                },
                None => continue,
            },
            None => continue,
        };
    }
}

fn main() {
    let arg:Vec<String> = env::args().collect();
    read_next("/".to_string(),&arg[1],&arg[2],&arg[3])
}
