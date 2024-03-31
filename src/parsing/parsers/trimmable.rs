pub trait Trimmable {
    fn trim(&self) -> Self;
}

impl Trimmable for String {
    fn trim(&self) -> Self {
        self.trim_end().to_string().trim_start().to_string()
    }
}

impl Trimmable for Option<String> {
    fn trim(&self) -> Self {
        match self {
            None => None,
            Some(str) => Some(str.trim().to_string().clone()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trimmable_string() {
        assert!("\n test string \t".to_string().trim().len() == 11);
    }

    #[test]
    fn test_trimmable_option_string() {
        assert!(None.trim() == None);
        assert!(Some("\n test string \t".to_string()).trim().unwrap().len() == 11);
    }
}
