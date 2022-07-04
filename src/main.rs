use dachterasse::{Config, Lecture, LectureClient};

fn main() {
    let mut client = LectureClient::with_config(Config::with_cache());

    print_lectures(&client.all_lectures());
    println!("==============");
    print_lectures_detailed(&client.all_lectures());

    println!("\n####################################################################################\n");
    print_lectures_detailed(&client.filter_lectures(vec!["HCGT: Human Computer Interaction & Computer Graphics Technology", "Professional Skills"]))
}

fn print_lectures(lectures: &[&Lecture]) {
    for lecture in lectures {
        println!("{}", lecture.title);
    }
}

fn print_lectures_detailed(lectures: &[&Lecture]) {
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