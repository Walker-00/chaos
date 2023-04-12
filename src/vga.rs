use core::fmt::{self /* for formatting stuff */, Write /* for writting stuff*/}; // We use core lib instead of std cuz core lib doesn't depend on operating system like std
use lazy_static::lazy_static; // we use lazy_static macro for declaring lazily evaluated statics
use spin::Mutex; // A primitive that synchronizes the execution of multiple threads
use volatile::Volatile; // Wraps a reference to make accesses to the referenced value volatile

const BUFF_HGT: usize = 25; // BUFFER MAX Heigh
const BUFF_WIT: usize = 80; // BUFFER MAX WIDTH

lazy_static! { // declaring evaluated writer static
    // making Mutex Writer
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        clmn_pst: 0, // start at column position 0
        color_code: ColorCode::new(Color::Cyan, Color::Black), // use Cyan Color as foreground and Black as background color
        buffer: unsafe { &mut *(0xb8000 as *mut Buff) }, // we gonna store address 0xb8000 which will be our text address as mutable Buff struct
    });
}

#[macro_export] // macro_export attribute export the macro to root crate so we can use it in anywhere in this project
                // macro_rules macro used to make macro
                // in this we gonna make print macro cuz no std no print macro ")
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::print(format_args!($($arg)*))); // this macro take arguments and call print func
}

#[macro_export] // read from above, I can't write again, I'm not usually write comment but this is for you guys to learn
                // like up but println print text and call new line
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*))); // like up but this call print macro with new line char
}

#[doc(hidden)] // doc(hidden) attribute to hide from the generated documentation
               // print func to write on screen it take arguments as param to write on screen
pub fn print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap(); // use WRITER static var to write on screen
}

#[allow(dead_code)] // this attribute allow unused code
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // derive attribute to impl Debug, Clone, Copy, PartialEq and Eq for Color enum
#[repr(u8)] // reqr(u8) attribute to explicitly specify the number for each color as u8
            // just read Color name and num it's fucking easy
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // like up but for ColorCode struct
#[repr(transparent)] // repr(transparent) attribute for ensure that the ColorCode has the exact same data layout as a u8
pub struct ColorCode(u8); // ColorCode struct contains the full color byte

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // like up but for ScreenChar struct
#[repr(C)] // repr(C) attribute guarantees that the struct’s fields are laid out exactly like in a C struct and thus guarantees the correct field ordering
pub struct ScreenChar {
    pub ascii_char: u8,        // field for ascii_char
    pub color_code: ColorCode, // field for color_code
}

#[repr(transparent)] // read up
pub struct Buff {
    pub chars: [[Volatile<ScreenChar>; BUFF_WIT]; BUFF_HGT], // for buffer chars
}

pub struct Writer {
    pub clmn_pst: usize,           // field for where to start write
    pub color_code: ColorCode,     // field for foreground and background color
    pub buffer: &'static mut Buff, // field for VGA buffer, 'static is a lifetime it's mean that buffer field is live the whole program run time
}

// impl Deref for ScreenChar to dereferencing operations
impl core::ops::Deref for ScreenChar {
    type Target = Self; // Target will be ScreenChar itself

    // deref func will return ScreenChar itself as deref
    fn deref(&self) -> &Self::Target {
        self
    }
}

// like up impl but it's mutable
impl core::ops::DerefMut for ScreenChar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

// impl ColorCode
impl ColorCode {
    // make func new for ColorCode which take foreground color and background as param and return ColorCode type
    pub fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

// impl Write trait for Writer to write string
impl fmt::Write for Writer {
    // write_str take self as mut and string s, that will write s's value to self and return Result
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// impl Writer struct
impl Writer {
    // write_byte take char as byte and write char to screen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // check the byte
            b'\n' => self.new_line(), // if byte is new line char call new_line fuc
            byte => {
                if self.clmn_pst >= BUFF_WIT {
                    self.new_line(); // if column position if greater than or equ with BUFFER MAX WIDTH call new_line func
                }

                let row = BUFF_HGT - 1; // decrease BUFFER MAX HIGHT
                let col = self.clmn_pst; // store self column position in col var

                let color_code = self.color_code; // store self color code in color_code var
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte, // write ascii char
                    color_code,       // write with color
                });

                self.clmn_pst += 1; // increase column position
            }
        }
    }

    // like up but it take string loop throught it and call write_byte for each byte
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            // we can use iter instead of for
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), // if byte is ascii write byte
                _ => self.write_byte(0xfe), // if byte is not ascii it will write ■
            }
        }
    }

    // just func for new_line printing, just read the code it's easy to read ")
    fn new_line(&mut self) {
        for row in 1..BUFF_HGT {
            for col in 0..BUFF_WIT {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char);
            }
        }
        self.clear_row(BUFF_HGT - 1);
        self.clmn_pst = 0;
    }

    // just func for clear the row
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFF_WIT {
            self.buffer.chars[row][col].write(blank);
        }
    }
}
