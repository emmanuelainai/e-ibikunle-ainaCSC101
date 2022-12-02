use std::io;
// fn student_(){
// }
fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    

    println!("What is your name");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let d:f32 = input4.trim().parse().expect("Not a valid number");

    println!("What is your School email");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let e:f32 = input5.trim().parse().expect("Not a valid number");

    println!("What is your Department");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let f:f32 = input6.trim().parse().expect("Not a valid number");

    println!("What is your State of origin");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    let g:f32 = input7.trim().parse().expect("Not a valid number");

    println!("Are you a class rep: yes(1.0) or no(0)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a = input1.trim().parse();

    println!("What is your CGPA");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("What is your current level");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");
    
    if a == 1.0{
        println!("You can vote");
        println!("{}",d);
        println!("{}",e);
        println!("{}",f);
        println!("{}",g);
    }else{
        println!("Sorry, you are not eligible to vote")
    }
    if b >= 4.0{
        println!("You can vote");
        println!("{}",d);
        println!("{}",e);
        println!("{}",f);
        println!("{}",g);
    }else{
        println!("Sorry, you are not eligible to vote")
    }
    if c > 100.0{
        println!("You can vote");
        println!("{}",d);
        println!("{}",e);
        println!("{}",f);
        println!("{}",g);
    }else{
        println!("Sorry, you are not eligible to vote")
    }
}