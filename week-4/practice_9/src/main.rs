fn main() {
    let a:f32 = 10.0;
    let b:f32 = 20.0;

    println!("Value of A:{}",a);
    println!("Value of B:{}",b);

    let mut res = a>b;
    println!("A greater than B: {}",res);

    res = a<b;
    println!("A greater than B: {}",res);

    res = a>=b;
    println!("A greater than or equal to B: {}",res);

    res = a<=b;
    println!("A lesser than or equal to B: {}",res);

    res = a==b;
    println!("A is equal to B: {}",res);

    res = a!=b;
    println!("A is not equal to B: {}",res);
}