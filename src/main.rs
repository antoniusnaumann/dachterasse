use dachterasse::LectureClient;

fn main() {
    let mut client = LectureClient::new();

    for lecture in client.lectures() {
        println!("{}", lecture.title);
        println!("{}", lecture.url);
        if let Some(c) = &lecture.categories {
            println!("{}: {:?}", lecture.title, c);
        }
        println!("--------------");
    }

    println!("==============");

    for lecture in client.lectures() {
        println!("{}", lecture.title);
    }
}
