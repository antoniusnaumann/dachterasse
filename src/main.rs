use dachterasse::{Config, Lecture, LectureClient};

fn main() {
    let mut client = LectureClient::with_config(Config::with_cache());

    print_lectures(client.lectures());
    println!("==============");
    print_lectures_detailed(client.lectures());
}

fn print_lectures(lectures: &Vec<Lecture>) {
    for lecture in lectures {
        println!("{}", lecture.title);
    }
}

fn print_lectures_detailed(lectures: &Vec<Lecture>) {
    for lecture in lectures {
        println!("{}", lecture.title);
        println!("{}", lecture.url);
        if let Some(c) = &lecture.categories {
            for (module, categories) in c {
                println!("> {}", module);
                for category in categories {
                    println!("\t {}", category);
                }
            }
        }
        println!("--------------");
    }
}