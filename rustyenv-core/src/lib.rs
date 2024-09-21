/// Indicates that the `rustyenv` project is currently in development.
///
/// # Examples
///
/// ```
/// use rustyenv::develop;
///
/// assert!(develop().is_ok());
/// ```
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully.
pub fn develop() -> Result<(), ()> {
    println!("`rustyenv` is in development.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(develop().is_ok());
    }
}
