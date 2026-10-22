fn main() -> Result<(), i32> {
    // in terminal will be displayed
    // Error: -11
    // echo $? => 1
    return Err(-11)?;
}
