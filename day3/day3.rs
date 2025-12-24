fn main(){
    let mut s1 = String::from("Hey Nigga!!");

    {
        let v1 = &mut s1;
        v1.push_str("Nig");
        println!("{}",v1);
    } // here v1 lives only in this scope {} so we can make many scopes and borrow s1 and do operations on it but it will only lives on there scope only

    {
        let v2 = &mut s1;
        v2.push_str("Nigger");
        println!("{}",v2);
    }

    //println!("{}",v2); this will cause error cause v2 only lives in it scope not outside of {}


    fn str_len(s: String) {
        println!("{}",s.len());
    }

    let s1 = String::from("Heyfsdfsdf");
    str_len(s1);
   // println!("{}",s);  this will cause error cause the str_len(s:String) which takes String and Sring dosent copy it uses ownsership so now the ownership of hey is with s in the function 

    //to fix  this issue we can use borrowing 

    fn str_len1(s:&String){ //use &String 
        println!("{}",s.len());
    }
    let s2 = String::from("HEY");
    str_len1(&s2); // pass the copy of &s, so "hey" is still owned by s
    println!("{}",s2.len()); 


    //the best way for this string operations are using &str they can support String,slices etc 
    
    fn str_len2(s: &str){
        println!("{}",s.len());
    }

    let s3: &str = "HEYYY";
    str_len2(s3);
    println!("{}",s3.len());

}