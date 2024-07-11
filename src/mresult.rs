#![allow(unused)]

enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
        MResult::Ok(value)
    }
    // Function to create an Err variant
    fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        match self {
            MResult::Ok(_) => true,
            MResult::Err(_) =>  false
        }
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        match self {
            MResult::Err(_) => true,
            MResult::Ok(_) =>  false
        }
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        match self {
            MResult::Ok(value) => value,
            MResult::Err(_) => panic!("Error value"),
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        match self {
            MResult::Ok(_) => panic!("Ok Value"),
            MResult::Err(error) => error,
        }
    }
}

// Add unit tests below
#[cfg(test)]
mod test {
    use crate::mresult::MResult;

    #[test]
    fn create_ok(){
        let ok:MResult<String,String> = MResult::ok("Success".to_string());

        if self::MResult::is_ok(&ok){
            println!("i can create ok")
        }
        if let value = self::MResult::unwrap(ok) {
            println!("{} was the value created", value)
        }
    }

    #[test]
    fn test_ok() {
        let result: MResult<i32, &str> = MResult::ok(42);
        assert!(result.is_ok());
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_err() {
        let result: MResult<i32, &str> = MResult::err("error");
        assert!(!result.is_ok());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "error");
    }


}
