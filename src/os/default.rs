use std::io::Write;

pub fn clear_line_move_one_up<W>(underlying: &mut W)
    where W: Write + Send + 'static
{
    let _ = write!(underlying, "\x1B[0A"); // Move the cursor up
    let _ = write!(underlying, "\x1B[2K\r");  // Clear the line
}
