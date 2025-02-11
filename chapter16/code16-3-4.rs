use std::fs;

fn main() -> std::io::Result<()> {
    fs::write("calvin_with_dad.txt",
    "Calvin: Dad, how come old photographs are always black and white? Didn't they
    have color film back then?
    Dad: Sure they did. In fact, those photographs *are* in color. It's just the
    *world* was black and white then.
    Calvin: Really?
    Dad: Yep. The world didn't turn color until sometimes in the 1930s...")?;
    Ok(())
}
