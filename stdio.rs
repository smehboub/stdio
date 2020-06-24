fn main() {
    use ::std::io::Write;
    let (stdout, stderr, errmsg) = (&mut ::std::io::stdout(), &mut ::std::io::stderr(), "Error writing to stderr");
    writeln!(stdout, "your stdout string").expect(errmsg);
    writeln!(stderr, "your stderr string").expect(errmsg);
}
