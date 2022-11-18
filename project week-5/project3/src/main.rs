use std::io;
fn main() { 
    let pricep:f32 = 3_200.0;
    let pricef:f32 = 3_000.0;
    let pricea:f32 = 3_500.0;
    let pricee:f32 = 2_000.0;
    let pricew:f32 = 2_500.0;
    let m = format!("{:^42}","Menu of food items available");
    let g = format!("{:^43}","Good day valued customer!");
    let g_1 = format!("{:^42}","What would you like to order today");
    let g_2 = format!("{:^42}","To order use the designated S/N");

    // MENU display
    println!("{m}");
    println!("  (S/N)         MENU                  PRICES");
    println!("(P)(1)  Poundo Yam & Edinkaiko Soup -  N3200");
    println!("\n(F)(2)  Fried Rice & Chicken 	  -  N3000");
    println!("\n(A)(3)  Amala & Ewedu Soup		  -  N2500");
    println!("\n(E)(4)  Eba & Egusi Soup		  -  N2000");
    println!("\n(W)(5)  White Rice & Stew		  -  N2500");
    println!("{g}"); //center greeting
    println!("{g_1}"); //center question
    println!("{g_2}"); //center command

     let mut input1 = String::new();
     let mut input2 = String::new();

    //  code for user input
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let foodtype:f32 = input1.trim().parse().expect("Not a valid string");

    if foodtype > 5.0{
        println!("Please pick from the S/N that is on the menu");
        panic!();
    }
    println!("How many portions would you like to order");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:f32 = input2.trim().parse().expect("Not a valid number");

    // price list
    if foodtype == 1.0{
        let total = quantity * pricep;
        // condition
        if total > 10000.0{
            let discount:f32 = total - (0.05 * total);
            println!("You get a 5% discount! Your new total is N{}",discount);
        }else{
            println!("Your total is N{}.\nHave a good rest of your  day",total);
        }
    }

// price of 2nd
else if foodtype == 2.0{
    let total = quantity * pricef;
    if total > 10000.0{
        let discount:f32 = total - (0.05 * total);
        println!("You get a 5% discount! Your new total is N{}",discount);
        }else{
            println!("Your total is N{}.\nHave a good rest of your day",total);
        }
}
// price of 3rd
else if foodtype == 3.0{
    let total = quantity * pricea;
    if total > 10000.0{
        let discount:f32 = total - (0.05 * total);
        println!("You get a 5% discount! Your new total is N{}",discount);
        }else{
            println!("Your total is N{}.\nHave a good rest of your day",total);
        }
}
//  price of 4th
else if foodtype == 4.0{
    let total = quantity * pricee;
    if total > 10000.0{
        let discount:f32 = total - (0.05 * total);
        println!("You get a 5% discount! Your new total is N{}",discount);
        }else{
            println!("Your total is N{}.\nHave a good rest of your day",total);
        }
}
// price of 5th
else if foodtype == 5.0{
    let total = quantity * pricew;
    if total > 10000.0{
        let discount:f32 = total - (0.05 * total);
        println!("You get a 5% discount! Your new total is N{}",discount);
        }else{
            println!("Your total is N{}.\nHave a good rest of your day",total);
        }
}
}