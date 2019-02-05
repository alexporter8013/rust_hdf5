pub fn return_true() -> bool {
		true
}


#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
        assert_eq!(return_true(), true);
    }
    
}
