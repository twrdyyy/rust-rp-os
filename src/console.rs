pub mod interface {
    pub use core::fmt::Result;
    pub use core::fmt::Arguments;

    pub trait Write {
        fn write_fmt(&self, args: Arguments) -> Result;
    }
       
          
    pub trait Statistics {    
        fn chars_written(&self) -> usize {
                  0
        }
    }
     
   /// '+' is the union of two traits :o
   pub trait All = Write + Statistics;
        
}