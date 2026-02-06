use crate::rcobj::RcObject;

///
/// A polymorphic allocator, dedicated on reducing size of the generated codes.
/// 
pub unsafe trait Alloc: RcObject {
    unsafe fn alloc(&self, size: usize, alignment: usize) -> *mut u8;
    unsafe fn release(&self, ptr: *mut u8, alignment: usize);
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        size: usize,
        alignment: usize,
        new_size: usize,
        new_alignment: usize,
    ) -> *mut u8;
}
