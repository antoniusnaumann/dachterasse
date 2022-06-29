#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod scrape; 
mod load; 

pub fn fetch_lectures() -> Vec<String> {
    Vec::new()
}

pub fn fetch_lecture_html() -> String {
    load::load_lecture_html("https://hpi.de/studium/im-studium/lehrveranstaltungen/it-systems-engineering-ma.html")
}
