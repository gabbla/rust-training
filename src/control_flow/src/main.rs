fn main() {
    let num = 3;

    if num < 5 {
        println!("true");
    } else {
        println!("false");
    }


    let my_var = if num < 5
    {
        4
    } else {
        6
    };

    // Using a loop to assign a variable value
    let mut cnt = 0;

    let res = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2; // <- this is assigned to res
        }
    };

    println!("res = {}", res); // 20 
    println!("cnt = {}", cnt ); // 10

    // Looping in through a collection
    let array: [u32; 4] = [0, 1, 2, 3];

    for elem in array.iter() {
        println!("Value: {}", elem);
    }
    
    println!("---------");

    // Looping n times
    for num in (1..4).rev() {
        println!("Value: {}", num);
    }   

}

