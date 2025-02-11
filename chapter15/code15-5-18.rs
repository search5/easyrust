// thiserror::Error의 이름이 'Error'와 같으므로 별칭을 제공합니다.
use std::error::Error as StdError;
use thiserror::Error;

#[derive(Error, Debug)]
enum SystemError {
    #[error("Couldn't send {0}")]
    SendError(i32),
    #[error("Something happened: {0}")]
    ExternalCrateError(String), // 외부 오류를 여기에 넣으세요.
}

trait ToSystemError<T> {
    fn to_system_error(self) -> Result<T, SystemError>;
}

impl<T, E: StdError> ToSystemError<T> for Result<T, E> {
    fn to_system_error(self) -> Result<T, SystemError> {
        self.map_err(|e| SystemError::ExternalCrateError(e.to_string()))
    }
}
