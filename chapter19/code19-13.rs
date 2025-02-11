fn not_done() {
    let time = 8;
    let reason = "lack of time";
    todo!("Not done yet because of {}. Check back in {} hours", reason, time);
}

fn main() {
    not_done();
}
