error_chain! {
    types {
        KairosError, KairosErrorKind, ResultExt, Result;
    }

    links {
    }

    foreign_links {
    }

    errors {

        UnknownError {
            description("Unknown Error")
            display("Unknown Error")
        }

    }

}
