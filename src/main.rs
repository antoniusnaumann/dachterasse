use dachterasse::LectureRepository;

fn main() {
    let mut repo = LectureRepository::new();
    for lecture in repo.lectures() {
        println!("{}", lecture.title);
        println!("{}", lecture.url);
        if let Some(c) = &lecture.categories {
            println!("{}: {:?}", lecture.title, c);
        }
        println!("--------------");
    }

    println!("==============");

    for lecture in repo.lectures() {
        println!("{}", lecture.title);
    }
}
