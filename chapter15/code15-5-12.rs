#[derive(Error, Debug)]
enum SystemError<'a> {
    #[error("Couldn't parse {0:?} into a str")]
    StringFromUtf8Error(&'a [u8]),
    #[error("Couldn't turn {0} into an i32")]
    ParseI32Error(&'a str),
    #[error("Couldn't send {0}; number is too large")]
    SendError(i32),
    #[error("Something happened")]
    OtherError,
}
