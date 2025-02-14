use proc_macros::{time_it, FieldCounter, Random, ValueAssigner};

#[derive(Debug, FieldCounter, Random, ValueAssigner)]
enum MyEnum {
    A,
    B,
    C,
    D,
    E,
}

#[time_it]
fn add(n1: usize, n2: usize) -> usize {
    let ans = n1 + n2;

    if ans < 5 {
        return ans;
    }

    ans
}

fn main() {
    let random: MyEnum = rand::random();
    println!(
        "{:?} {} {} {:?}",
        random,
        MyEnum::field_count(),
        random.get_value(),
        MyEnum::get_type(2)
    );

    let res = add(1, 2);

    println!("Addition result: {res}");
}
