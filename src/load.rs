pub fn load_lecture_html(url: &str) -> String {
    reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap()
}

