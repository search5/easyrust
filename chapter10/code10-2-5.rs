enum FileState {
    CannotAccessFile,
    FileOpenedAndReady,
    NoSuchFileExists,
    SimilarFileNameInNextDirectory,
}

fn give_filestate(input: &FileState) {
    use FileState::{
        CannotAccessFile as NoAccess,
        FileOpenedAndReady as 잘됨,
        NoSuchFileExists as NoFile,
        SimilarFileNameInNextDirectory as OtherDirectory
    };

    match input {
        잘됨 => println!("Here is your file"),
        NoAccess => println!("Can't access file."),
        NoFile => println!("Sorry, there is no file by that name."),
        OtherDirectory => println!("Please check the other directory."),
    }
}

fn main() {}
