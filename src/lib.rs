#[macro_export]
macro_rules! refute {
    ($val:expr) => {
        assert!(!$val)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        refute!(false);
    }
}
