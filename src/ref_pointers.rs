// Reference pointers point to a resource in memory.

pub fn run() {
    // Variables of primitive data types, when assigned to another variable,
    // nothing bad happens. Both variables will contain the value.
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // However, with non-primitive data types, when assigning a variable to
    // another one, the first one will no longer hold the value. A "move" is
    // performed.

    // To point to the value of a non-primitive data type variable, you need
    // to use a reference, which is defined using the ampersand symbol '&'
    // before the variable name.
    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}