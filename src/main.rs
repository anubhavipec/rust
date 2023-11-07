fn main() {
    println!("Hello, world!");
    //option_example();
    //ownership_and_moves_example();
    //mutability_example();
    // borrow_checker_example();
    //mutability_borrow_example();
    aliasing_example();
}

//TODO:=== ALIASING START=====

struct Point{x: i32,y: i32,z: i32}

fn aliasing_example(){
    let mut point = Point{x:0,y:0,z:0};

    let borrowed_point = &point;
    let another_borrow = &point;

    // Error! Can't borrow `point` as mutable because it's currently
    // borrowed as immutable.
    //let mutable_borrow = &mut point;

    //below we are using immutable borrowed value again hence above mutable borrow will fail compilation
    println!("Immutable borowwed value are being used here again {}",borrowed_point.x);


    // The immutable references are no longer used for the rest of the code so
    // it is possible to reborrow with a mutable reference.
    let mutable_borrow = &mut point;

    mutable_borrow.x = 2;

    let y = &point;


    println!("Point has coordinates: ({}, {}, {})",
             mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);
}


//TODO:=== ALIASING END=======
//TODO:===MUTABILITY START=====
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book{
    // &'static str is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book){

    println!("Immuatable borrowed book {}",book.title);
}

//this function takes a reference to a mutable book and changes its value
fn new_edition(book: &mut Book){
    book.year = 2034;
    println!("Borrowed mutable reference and changed it {}",book.year);
}

fn mutability_borrow_example(){

    // create an immutable book name below
    let immutabook = Book {
        author: "Douglas Hosfstdar",
        title: "Bach",
        year: 1979,
    };

    //create a mutable copy
    let mut mutabook = immutabook;

    borrow_book(&immutabook);
    borrow_book(&mutabook);

    new_edition(&mut mutabook);

}




//TODO:===MUTABILITY END=====


//TODO:====BORROWING START=====

fn borrow_checker_example(){
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        //takes a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        //below will give error
        // cannot destroy boxed_i32 while the inner values is borrowed later in scope
        //eat_box_i32(boxed_i32);

        //this attemps to borrow _ref_to_i32 after the inner value is destroyed
        borrow_i32(_ref_to_i32);

    }

    //boxed_i32 can now give ownership to eat_box and be destroyed
    eat_box_i32(boxed_i32);

}

//this function borrows in i32
fn borrow_i32(borrowed_i32: &i32){
    println!("This int is : {} borrowed ",borrowed_i32);
}

//This function takes ownership of the box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>){
    println!("Destroying box that contains {}",boxed_i32);
}


//TODO:====BORROWING END=======

//TODO:=====MUTABILITY START==============

fn mutability_example(){
    let immutable_box = Box::new(5u32);

    //mutablity error
    //*immutable_box = 4;

    //move the box , changing the ownership and mutablity

    let mut mutable_box = immutable_box;

    *mutable_box = 4;

    println!("Mutable box contains {}",mutable_box);


}

//TODO:=====MUTABILITY END==============

//TODO:==========OWNERSHIP AND MOVES START ============

fn ownership_and_moves_example(){

    //stack allocated integer
    let x = 5u32;

    // Copy x to y -> no resources are moved here
    let y = x;
    println!("Both values can be used independently x: {} y: {}",x,y);

    //  a is pointer to heap allocated integer
    let a = Box::new(5i32);

    //Move a into b
    let b = a;

    /*
    The pointer address of a is copied(not the data) into b
    Both are now pointers to same heap allocated data but now b owns it
    a can no longer access the data because it no longer owns the memory.
     */
    //below line will fail
    //println!("a contains {}",a);
    destroy_box(b);

    /*
    since the heap memory has been freed at this point, this action would
     result in dereferencing freed memory, but its forbidden by compiler

     */
    //below line will fail
    //println!("b contains {}",b);



}

fn destroy_box(c: Box<i32>){
    println!("Destroying a box that contains {}",c);
    // c is destroyed and the memory freed
}


//TODO:==========OWNERSHIP AND MOVES ENDS ============

//TODO:===============OPTION EXAMPLE START========

//below is integer division that does not panic
// using Option<T> enum
fn checked_division(dividend: i32,divisor: i32) -> Option<i32>{
    if divisor == 0 {
        None
    } else {
        Some(dividend/divisor)
    }
}


//function handling division which may not succeed

fn try_division(dividend: i32, divisor: i32){
    // `Option` values can be pattern matched
    match checked_division(dividend,divisor){
        None => println!("Failed division"),
        Some(quo) => {
            println!("Answer {}",quo)
        },
    }
}


fn option_example(){
    try_division(4,2);
    try_division(1,0);

    let _equivalent_none = None::<i32>;
}

//TODO:===============OPTION EXAMPLE END========

