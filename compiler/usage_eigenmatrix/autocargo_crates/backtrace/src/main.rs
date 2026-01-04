// PROCESSED BY: unified-build
// SOURCE: library/backtrace/examples/backtrace.rs

// @transform(id=REMOVE_UNUSED, desc="Remove unused private items", applied=true)
# ! [feature (array_windows)] # ! [feature (core_io_borrowed_buf)] # ! [feature (if_let_guard)] # ! [feature (staged_api)] # ! [feature (rustc_private)] use std :: collections :: HashMap ; use rustc_index :: newtype_index ; use backtrace :: Backtrace ; fn main () { println ! ("{:?}" , Backtrace :: new ()) ; }