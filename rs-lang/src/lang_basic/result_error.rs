use std::error::Error;
use std::fs::File;
use std::io::BufRead;

/// 最简单的方式实现Error
#[derive(Debug)]
pub struct MyError(String);

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MyError {}

impl From<String> for MyError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// Io Error 映射到 MyError
pub fn open_file1(file_name: &str) -> Result<File, MyError> {
    File::open(file_name).map_err(|e| MyError(format!("fail to open file: {}", e)))
}

/// MyError实现了From<String>， 通过 ? 运算符可以把String直接转为MyError
pub fn open_file2(file_name: &str) -> Result<File, MyError> {
    let f = File::open(file_name).map_err(|e| format!("fail to open file: {}", e))?;
    Ok(f)
}
///
/// 定义一个新的Error，聚合多种不同Error并保留内部Error
#[derive(Debug)]
pub enum CustomError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    General(String),
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::Io(e) => write!(f, "IO error: {}", e),
            CustomError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            CustomError::General(s) => write!(f, "General error: {}", s),
        }
    }
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CustomError::Io(e) => Some(e),
            CustomError::Utf8(e) => Some(e),
            CustomError::General(_) => None,
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::string::FromUtf8Error> for CustomError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

const MAX_LENGTH: usize = 1024;
pub fn read_first_line(file_name: &str) -> Result<String, CustomError> {
    let file = File::open(file_name).map_err(|e| CustomError::Io(e))?;
    let mut reader = std::io::BufReader::new(file);
    let mut buf: Vec<u8> = vec![];
    let len = reader
        .read_until(b'\n', &mut buf)
        .map_err(CustomError::Io)?;
    // 已经为CustomError实现From<std::string::FromUtf8Error>,不用显式调用map_err
    let result = String::from_utf8(buf)?;
    if result.len() > MAX_LENGTH {
        return Err(CustomError::General(format!("Line too long: {len}")));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    #[test]
    fn t() {
        let r: Result<i32, String> = Ok(123);
        // `Result<T, E>` to `Result<T, F>`
        r.clone().map_err(|e| format!("{}", e)).unwrap();
        // `Result<T, E>` to `Result<U, E>`
        r.clone().map(|t| t * 2).unwrap();
        r.clone().and(Ok(111)).unwrap();
        r.clone().and_then(|_| Ok(111)).unwrap();
        r.clone().or(r.clone()).unwrap();
        r.clone().or_else(|u| Err(u)).unwrap();
    }
}
