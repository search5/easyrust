use std::env::VarError;
use std::error::Error;
use std::fmt;

enum MyError {
    TooMuchStuff,
    CantConnect,
    NoUserRegistered,
    SomethingElse,
}

impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Wouldn't you like to know...")
    }
}

impl fmt::Debug for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lol not telling you what went wrong").finish()
    }
}

// 두 가지 오류 중 하나를 제공하는 함수입니다.
fn give_error_back(choice: bool) -> Box<dyn Error> {
    if choice {
        Box::new(MyError::TooMuchStuff)
    } else {
        Box::new(VarError::NotPresent)
    }
}

fn main() { // 두 종류의 오류가 있는 벡터를 만듭니다.
    let errs = [true, false, false, true]
        .into_iter()
        .map(|boolean| give_error_back(boolean))
        .collect::<Vec<_>>();
    
    // 오류를 출력합니다.
    println!("{errs:#?}");
    
    // 참조를 사용하므로 downcast_ref를 사용합니다.
    for err in errs.iter() {
        if let Some(my_error) = err.downcast_ref::<MyError>() {
            println!("Got a MyError!");
            // match 문을 사용해 MyError의 각 배리언트에 맞는
            // 작업을 수행할 수 있습니다.
        } else if let Some(parse_error) = err.downcast_ref::<VarError>() {
            println!("Got a VarError!");
            // 이제 오류가 구체적이므로 오류에 맞는 원하는 작업을 수행할 수 있습니다.
        }
    }
}
