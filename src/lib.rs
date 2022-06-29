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

pub fn fetch_lectures() -> Vec<(String, String)> {
    scrape::fetch_lectures()
}
