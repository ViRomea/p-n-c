use std::io;
use std::convert::TryInto;


fn trash2() {
    


    let mut a: Vec<i8> = vec![2,6,16,5];
    let b: Vec<i8> = vec![6,2,8,85];  
    v8v8v16(&a, &b);


//    a = <Vec<i8> as Into<Vec<i16>>>::into(a);
    let c: Vec<i16> = vec![2,16,3,5,2,65,300,84];
    v163v16(&c);
}






fn trash() -> io::Result<()> {
    let mut a = String::new();
    io::stdin().read_line(&mut a)?;
    println!("{}", a);


    let s = Square{c: 50.0, b: 40.0};
    println!("{}",area(s.c, s.b));
    println!("Hi Rust");

    build_user(String::from("U1"), 1);

    Ok(())
}

fn area(n: f32, m: f32) -> f32 {
    let k = n * m;    
    (k).into()
}


fn build_user(name: String, id: i16) -> User {
    
    let a = User {
        name: name,
        id:   id,
};
    println!("{}, {}", a.name, a.id);
    a
}


struct User {
    name: String,
    id:   i16,
    health: u8
}
        


struct Square {
    c: f32,
    b: f32,
}


fn v8v8v16(a: &Vec<i8>, b: &Vec<i8>) -> Vec<i16> {
    let mut c: Vec<i16> = Vec::new();

    for r in a 
    {
        for n in b 
        { 
            
            let k: i16 = <i8 as Into<i16>>::into(*(r)) * <i8 as Into<i16>>::into(*(n));                                  
            c.push(k);
        }
    }
    println!("{:?}", c);
    c
}


fn v163v16(a: &Vec<i16>, ) -> Vec<i16> {
    let mut b: Vec<i16> = Vec::new();
    
    for r in 0..=a.len()-1 
    {
        if ((r + 1) % 3) == 0
        {
         println!("{}", a[r]);
        b.push(r.try_into().unwrap())    
        }
    }
    println!("{:?}", b);
    b
}   
    























