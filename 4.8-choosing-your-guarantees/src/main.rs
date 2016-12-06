fn test_box() -> Box<usize> {
    // Boxes allocate on the heap, and represent a pointer to that
    // data and are handled mostly for us out-of-the-box (pun intended).
    let x = Box::new(1);
    // Transfer ownership to y
    let y = x;
    // x is no longer accessible here

    // If we don't return the box here, moving it out of scope
    // to the caller, then the destructors are run, and cleaned up,
    // freeing up our memory. In this case we return the box so our
    // main function now owns the box.
    return y;
}

fn test_immutable_and_mutable_refs() {
    // These references follow the "read-write lock" pattern, meaning
    // that they may either have one of the following, but not both:
    // * one or more references (&T) to a resource.
    // * exactly one mutable reference (&mut T)
    // This is enforced at compile time! Whoa! Usually used for
    // sharing cheap references around.

    let mut x = 5;
    {
        let y = &mut x; // -+ &mut borrow starts here
        *y += 1;        //  |
    }                   // -+ ... and ends here

    println!("{}", x);  // <- try to borrow x here
}

fn main() {
    // Box<T>
    let x = test_box();

    // &T, &mut T
    test_immutable_and_mutable_refs();
    // Rc<T>
}
