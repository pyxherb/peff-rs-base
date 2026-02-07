#![cfg_attr(not(feature = "std"), no_std)]

mod alloc;
mod list;
mod ptr;
mod rcobj;
pub mod boxing;

#[cfg(test)]
mod tests {
    use crate::{alloc::StdAlloc, list::List};

    use super::*;

    #[test]
    fn it_works() {
        let mut allocator = StdAlloc::new();
        let mut ls = List::<i32>::new(allocator.into_ptr_mut());

        for i in 1..100 {
            ls.push_back(i);
        }

        for i in ls.begin() {
            println!("{}", i);
        }
    }
}
