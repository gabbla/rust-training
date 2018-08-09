fn main() {
    // This statement resutl in an error
    // let x = (let y = 6);

    let y = 67;

    let x = {
        let y = 5;
        6 + y
    };

    println!("X: {}", x);

    let res = sum(4, 6);
    println!("The sum 4 + 6 = {}", res);

    let weired_res = weired_sum(4, 6);
    println!("The weired sum 4 + 6 =  {}", weired_res);

}

fn sum(x: u32, y: u32) -> u32 {
    x + y
}

fn weired_sum(x: u32, y: u32) -> u32 {
    // I had to redefine a temp var since x is not mut
    let mut xx = x;
    xx = x + y / 2;
    x + y - x * 2
}
