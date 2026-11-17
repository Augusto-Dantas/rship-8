#[allow(dead_code)]

/* 
 * TODO: implement integer overflow handling with the wrapping method 
 */
mod Chip8Interpreter {
    pub struct Chip8 {
        const BYTES_SIZE_IN_BITS:       u8 = 8,

        const DISPLAY_HEIGHT_IN_PIXELS: u8 = 32,
        const DISPLAY_WIDTH_IN_PIXELS:  u8 = 64,

        const MEMORY_SIZE_IN_BYTES:     u16 = 4096 * BYTES_SIZE_IN_BITS,
        const DISPLAY_SIZE_IN_PIXELS:   u16 = 64 * 32,

        const DELAY_TIMER_DECREMENT_RATE_IN_HZ: u8 = 60

        let mut memory:             [u8, MEMORY_SIZE_IN_BYTES],
        let mut display:            [bool, DISPLAY_SIZE_IN_PIXELS],
        let mut mutprogramCounter:  u16,
        let mut indexRegister:      u16,
        let mut stack:              Vec::new(),
        let mut delayTimer:         u8,
        let mut soundTimer          u8,
        let mut registers           [u16, 0xF],
    }

    impl Chip8 {

        fn process_opcode() {
        }

    }
}




    
