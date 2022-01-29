pub mod rr_error {
    use std::error::Error;
    use std::fmt;
    use wasm_bindgen::prelude::*;

    #[derive(Debug)]
    #[wasm_bindgen]
    pub struct RrError {
        details: String,
    }

    impl RrError {
        fn new(msg: &str) -> RrError {
            RrError {
                details: msg.to_string(),
            }
        }
    }

    impl fmt::Display for RrError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl Error for RrError {
        fn description(&self) -> &str {
            &self.details
        }
    }
}
