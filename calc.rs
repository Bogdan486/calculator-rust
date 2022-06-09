use std::io;
fn input_int(x: &mut i32){
    let mut _s=String::new();
    io::stdin().read_line(&mut _s).expect("ERROR");
    let d:i32;
    d=_s.trim().parse().unwrap();
    *x=d;
}
fn add(n:i32)->i32{
    if n==0{
        return 0;
    }
    let mut a:i32=0;
    input_int(&mut a);
    return a+add(n-1);
}
fn sub(n:i32,x:&mut i32){
    if n==0{
        return;
    }
    let mut a:i32=0;
    input_int(&mut a);
    *x -=a;
    sub(n-1,x);
}
fn inm(n:i32)->i32{
    if n==0{
        return 1;
    }
    let mut a:i32=0;
    input_int(&mut a);
    return a*inm(n-1);
}
pub fn run(){
    println!("Calculator realizat in rust");
    println!("De Dinca Bogdan Catalin");
    println!("1.Adunare");
    println!("2.Scadere");
    println!("3.Inmultire");
    let mut x:i32=0;
    input_int(&mut x);
    if x==1 {
        println!("Cate numere?");
        let mut n:i32=0;
        input_int(&mut n);
        let s:i32=add(n);
        println!("Rezultatul este {}",s);
    }
    if x==2 {
        println!("Cate numere?");
        let mut n:i32=0;
        input_int(&mut n);
        println!("Numarul din care vreti sa scadeti");
        let mut a:i32=0;
        input_int(&mut a);
        sub(n,&mut a);
        println!("Rezultatul este {}",a);
    }
    if x==3 {
        println!("Cate numere?");
        let mut n:i32=0;
        input_int(&mut n);
        let s:i32=inm(n);
        println!("Rezultatul este {}",s);
    }
}