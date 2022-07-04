use dachterasse::{Config, LectureClient};

fn main() {
    let mut client = LectureClient::with_config(Config::with_cache());

    for lecture in client.lectures() {
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

    println!("==============");

    for lecture in client.lectures() {
        println!("{}", lecture.title);
    }
}
