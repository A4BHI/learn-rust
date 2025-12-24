fn main(){
    let mut s1 = String::from("Hey Nigga!!");

    {
        let v1 = &mut s1;
        v1.push_str("Nig");
        println!("{}",v1);
    }

    


}