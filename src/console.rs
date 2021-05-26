/// Console interface
pub mod interface {
    pub use core::fmt::Result;
    pub use core::fmt::Arguments;

    /// Write functions
    pub trait Write {
        fn write_char(&self, c: char);
        fn write_fmt(&self, args: Arguments) -> Result;
        fn flush(&self);
    }

    /// Read functions
    pub trait Read {
        fn read_char(&self) -> char {
            ' '
        }
        fn clear_rx(&self);
    }

   /// Union of Write and Read traits
   pub trait All = Read + Write;
        
}