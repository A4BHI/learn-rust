fn main(){
    println!("Hari Sree GanaPadhaye Namah");
    let num: i32 = 5;
    println!("{}",num);
    let s =String::from("Heyyy");
    println!("{s}");

    let mut tup:(i32,&str,f32)=(100,"HEYY Bruh",0.21);
    println!("{:#?}",tup);
    tup.0=200;
    println!("{:?}",tup);
}