fn main() {

    ///////////////////////////////////////////////////////////////////////
    
    // MATCHING REFERENCE VS. VALUE
    let reference = &4;
 
    // Match the reference itself
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // Match the value: dereference before matching
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    ///////////////////////////////////////////////////////////////////////

    // USE 'REF' TO MATCH REFERENCE TO ACTUAL VALUE

    // Can create a reference with 'ref' as well
    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    
    let value = 5;
    let mut mut_value = 6;

    // Match reference
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Math reference first, then dereference to actually edit it
    match mut_value {
        ref mut m => {
            // Derference before editing
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    ///////////////////////////////////////////////////////////////////////

}
