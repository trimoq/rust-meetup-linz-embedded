fn main(){
    
    basic();

    endianness();

    size();

}

fn basic(){
    println!("Sample basic------------");
    let num = 42u16;
    println!("num: {}", num);
    let brw = &num;
    let ptr = brw as *const u16;
    
    println!("ptr: {:p}", ptr);

    unsafe{
        println!("*ptr: {}", *ptr);
    }

}

// Pay attention to the correct types when casting pointers
// The behavior you see here may change from platform to platform
fn endianness(){
    println!("Sample endianness------------");
    let num = 0x01_02_u16;
    println!("num: {}", num);
    let pointer = &num as *const u16;
    println!("Pointer: {:p}", pointer);
    unsafe{
        println!("*pointer: {}", *pointer);
    }

    // disallowed
    //let pointer = &num as *const u8;

    // bad idea but allowed
    let p1 = &num as *const u16 as u64 as *const u8;
    let p2 = (&num as *const u16 as u64 +1 ) as *const u8;

    println!(" p1: {:p}", p1);
    println!(" p2: {:p}", p2);

    unsafe{
        println!("*p1: {}", *p1);
        println!("*p2: {}", *p2);
    }
}

// Pay attention to the correct types when casting pointers
// The result you see here, changes with each invocation
fn size(){
    println!("Sample size------------");
    let num = 0x01_02_u16;
    println!("num: {}", num);

    // pay attention to your types
    let ptr = &num as *const u16 as u64 as *const u128;

    println!(" ptr: {:p}", ptr);

    unsafe{
        println!("*ptr: {}", *ptr);
    }
}