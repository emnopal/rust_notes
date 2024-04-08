/*
NOTE: this is rust important part
To manage the memory, rust use ownership concept

Rules of ownership:
1. On every value in rust must have owner (variable that had value)
2. In the same time, there is only 1 owner
3. When owner out from the scope, the value will be deleted
*/

// this example of ownership in rust

#[test]
fn ownership_rules() {
    // let's say we have variable a
    // a can't be accessed here, because it's not already declared
    
    // then we declare a
    let a = 10; // a can be accessed here

    { // b can't be accessed here. because it's not already declared
        let b = 20; // b can be accessed here
        println!("{}", b);
    } // scope of b finished, b deleted, b can't be accessed again since it was deleted from the memory

    println!("{}", a);
} // scope of a finished, a deleted, a can't be accessed again since it was deleted from the memory

/*
Data Copy:

Based on ownership in rust, in the same time, there is only one owner

so when we interact the data, the data will only have one owner

in every fixed size (stack) data, when we adding to another variable (new owner)
the data will be copied from old variable (old owner) to new variable (new owner),
so new variable (new owner) will have copied old variable (old owner) data

==================================
before copy
data (valid)

when copy happens
data ---> copied_data

after copy
data (valid) - copied_data (valid)
===================================
*/

#[test]
fn data_copy() {
    let mut a = 10; // original owner of 10 is a
    let b = a; // this is copy of a, 10. so b own copied value of a, 
    // thus a != b (by rule terms, not by boolean), so in every change of a, b will not changed

    println!("{}, {}", a, b);

    // we do some changes in a 
    a += 10;

    println!("{}, {}", a, b); // b not change
}

/*
Ownership movement:

data copy not happens in Heap,
the rule still same, one time one owner
but the terms is bit different

instead of copy, in Heap, the data is doing transfer ownership
from old owner to new owner

after transfer finished, automatically old owner is invalid

===============================================
before transfer ownership
data (valid)

when transfer ownership happen
data (valid) ---> (ownership transfer) new_data

after transfer ownership
data (invalid) - new_data (valid)
================================================
*/

#[test]
fn ownership_movement() {
    let name1 = String::from("Alexander");

    // transfer ownership
    let name2 = name1; // value moved here

    // println!("{}", name1); // this will be error, since name1 not accessed
    println!("{}", name2);
}

// conclusion: Stack will copy and Heap will transfer the ownership

/*
so how the heck if we want to copy Heap?
the answers is `Clone`, using clone() method

clone basically imitating the data of heap, like what copy did in Stack
and in every data whose stored by Heap, must have clone() method
*/

#[test]
fn heap_clone() {
    let name1 = String::from("Alexander");

    // transfer ownership
    let name2 = name1.clone(); // value copied here

    println!("{}", name1); // this will not error anymore
    println!("{}", name2);
}