use dachterasse;
use dachterasse::LectureScraper;

fn main() {
    let scraper = LectureScraper::new();
    let lectures = scraper.fetch_lectures();
    for lecture in &lectures {
        println!("{}", lecture.title);
        println!("{}", lecture.url);
        println!("--------------");
    }

    for lecture in &scraper.fetch_lecture_details(Some(lectures)) {
        println!("{:?}", lecture.categories);
    }
}
