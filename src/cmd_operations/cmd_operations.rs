pub fn clear_cmd()
{
    print!("{esc}c", esc = 27 as char);
}