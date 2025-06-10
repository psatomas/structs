struct ShortDuration(u32, u32);
struct LongDuration(u32, u32);
fn main() {
    let work_shift = ShortDuration(8, 0);
    println!("{} hours and {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years and {} months", era.0, era.1);
}

fn go_to_work(length: ShortDuration) {
    println!("Passing time {} hours and {} minutes", length.0, length.1);
}

fn accept_tuple(t: (u32, u32)) {}