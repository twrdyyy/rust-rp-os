pub mod interface {
    pub use core::fmt::Result;
    pub use core::fmt::Arguments;

    pub trait Write {
        fn write_char(&self, c: char);
        fn write_fmt(&self, args: Arguments) -> Result;
        fn flush(&self);
    }
    
    
    pub trait Read {
        fn read_char(&self) -> char {
            ' '
        }

        fn clear_rx(&self);
    }
          
    pub trait Statistics {    
        fn chars_written(&self) -> usize {
                  0
        }
        fn chars_read(&self) -> usize {
            0
        }
    }
     
   /// '+' is the union of two traits :o
   pub trait All = Read + Write + Statistics;
        
}