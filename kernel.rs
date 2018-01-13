extern crate console;
use console::{print,clean};

#[no_mangle]
pub extern "C" fn kmain() {
    /*let iter=IterateMaps{index:0};
    for my in iter{
        test(my);
    }*/
    clean(1);
    print();
}

