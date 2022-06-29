use dachterasse;
use dachterasse::LectureScraper;

fn main() {
    let scraper = LectureScraper::new();
    for lecture in scraper.fetch_lectures() {
        println!("{}", lecture.0);
        println!("{}", lecture.1);
        println!("--------------");
    }
}
