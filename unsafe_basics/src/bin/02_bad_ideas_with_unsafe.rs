fn main(){
    
    // nullptr();

    mutated();

    happy_little_accidents();

    unhappy_big_accidents();

    havoc();

}

fn nullptr(){
    println!("Sample nullptr ------");

    let ptr = 0 as *const u16;    
    println!("ptr: {:p}", ptr);
    unsafe{
        println!("*ptr: {}", *ptr);
    }

}

// This shows that a immutable piece of memory can accidentally become mutable through pointers if not taken care of
fn mutated(){
    println!("Sample mutated ------");

    let num = 0u8;

    // disallowed:
    // num+=1;

    //also disallowed
    // let ptr = &num as *mut u8;

    // bad idea
    let ptr = &num as *const u8 as u64 as *mut u8;
    println!("num: {}", num);
    unsafe{ *ptr +=1};
    println!("num: {}", num);

}


// We can have multiple *mut to one variable
fn happy_little_accidents(){
    println!("Sample happy_little_accidents ------");

    let mut num = 0u8;

    // let r0 = &mut num;
    // let r1 = &mut num;

    // *r0 += 1;
    // *r1 += 1;

    // println!("r0: {}", r0);
    // println!("r1: {}", r1);

    let p0 = &mut num as *mut u8 ;
    let p1 = &mut num as *mut u8 ;

    unsafe{
        *p0 += 1;
        *p1 += 1;
        println!("r0: {}", *p0);
        println!("r1: {}", *p1);
    } 

}

// Attention: If you create mutable references like this,
//  you leak unsafe behavior out of you code!
fn unhappy_big_accidents(){

    let mut num = 0u64;

    let p0 = &mut num as *mut u64 ;
    let p1 = &mut num as *mut u64 ;

    let r0;
    let r1;

    unsafe{ 
        r0 = p0.as_mut().unwrap();
        r1 = p1.as_mut().unwrap();
    };

    *r0 += 1;
    *r1 += 1;

    println!("num: {}", num);

}




