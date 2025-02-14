use proc_macros::{time_it, FieldCounter, Random};

#[derive(Debug, FieldCounter, Random)]
enum MyEnum {
    A,
    B,
    C,
    D,
    E,
}

// impl Distribution<MyEnum> for StandardUniform {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MyEnum {
//         match rng.random_range(0..5) {
//             0 => MyEnum::A,
//             1 => MyEnum::B,
//             2 => MyEnum::C,
//             3 => MyEnum::D,
//             _ => MyEnum::E,
//         }
//     }
// }

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
    println!("{:?} {}", random, MyEnum::field_count());

    let res = add(1, 2);

    println!("Addition result: {res}");
}
