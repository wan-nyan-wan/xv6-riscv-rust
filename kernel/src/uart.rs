use core::{fmt::{Error, Write}};

pub struct Uart {
    base: u64,
}

static RHR: usize = 0; // receive holding register (for input bytes)
static THR: usize = 0; // transmit holding register (for output bytes)
static IER: usize = 1; // interrupt enable register
static FCR: usize = 2; // FIFO control register
static ISR: usize = 2; // interrupt status register
static LCR: usize = 3; // line control register
static LSR: usize = 5; // line status register

impl Write for Uart {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        for c in out.bytes() {
            self.put(c);
        }
        Ok(())
    }
}

impl Uart {
    pub fn new(base: u64) -> Self {
        Uart { base }
    }

    pub fn init(&mut self) {
        self.write_reg(IER, 0x00);
        self.write_reg(LCR, 0x80);
        self.write_reg(0, 0x03);
        self.write_reg(1, 0x00);
        self.write_reg(LCR, 0x03);
        self.write_reg(FCR, 0x07);
        self.write_reg(IER, 0x01);
    }

    pub fn write_reg(&mut self, reg: usize, val: u8) {
        unsafe {
            let ptr = self.base as *mut u8;
            ptr.add(reg).write_volatile(val);
        }
    }
    
    pub fn read_reg(&mut self, reg: usize) -> u8 {
        unsafe {
            let ptr = self.base as *mut u8;
            ptr.add(reg).read_volatile()
        }
    }

    pub fn put(&mut self, c: u8) {
        while (self.read_reg(LSR) & (1 << 5)) == 0 {}
        self.write_reg(THR, c);
    }

    pub fn get(&mut self) -> Option<u8> {
        let ptr = self.base as *mut u8;
        unsafe {
            if ptr.add(5).read_volatile() & 1 == 0 {
                    None
            }
            else {
                    Some(ptr.add(0).read_volatile())
            }
        }
    }
}
