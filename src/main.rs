use dachterasse;

fn main() {
    for lecture in dachterasse::fetch_lectures() {
        println!("{}", lecture.0);
        println!("{}", lecture.1);
        println!("--------------");
    }
}
