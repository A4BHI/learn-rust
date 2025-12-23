fn main(){
    let x = 10;
    let mut y =20;
    y=x+y;
    println!("x={} y={}",x,y);

    let a =10;
    let b = a;
    println!("{}",a); //No error i32 copy no ownership change 


    let s1=String::from("String 1");
    let s2=s1;
    // println!("{}",s1) //Error for string ownership changes so s1 becomes unusable
    println!("{}",s2); //this works because now s2 owns the value that was inside s1 before



    //For fixing the above scenario we can use borrowing like this

    let str1 = String::from("String 1");
    let str2=&str1; //here str2 borrows str1 value using & but still str1 is the owner and str2 is borrower so borrower can change the value
    println!("str1: {} , str2: {}",str1,str2); 
}