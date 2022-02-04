pub(crate) trait FromFile<T> {
    fn from_file(x: T) -> Self;
}

impl FromFile<&str> for Vec<String> {
    fn from_file(x: &str) -> Vec<String> {
        let data = std::fs::read_to_string(x).expect("Unable to read file");
        data.lines()
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.to_string())
                }
            })
            .collect::<Vec<String>>()
    }
}

impl FromFile<String> for Vec<String> {
    fn from_file(x: String) -> Self {
        Vec::from_file(&x)
    }
}

impl FromFile<&String> for Vec<String> {
    fn from_file(x: &String) -> Vec<String> {
        let data = std::fs::read_to_string(x).expect("Unable to read file");
        data.lines()
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    Some(x.to_string())
                }
            })
            .collect::<Vec<String>>()
    }
}
