fn main(){
    let mut s1 = String::from("Hey Nigga!!");

    {
        let v1 = &mut s1;
        v1.push_str("Nig");
        println!("{}",v1);
    } // herer v1 lives only in this scope {} so we can make many scopes and borrow s1 and do operations on it but it will only lives on there scope only

    {
        let v2 = &mut s1;
        v2.push_str("Nigger");
        println!("{}",v2);
    }

    //println!("{}",v2); this will cause error cause v2 only lives in it scope not outside of {}


    


}