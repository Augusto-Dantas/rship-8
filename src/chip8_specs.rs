#[allow(dead_code)]

/* 
 * TODO: implement integer overflow handling with the wrapping method 
 */
mod chip8_specs {
    struct Chip8 {
        const DISPLAY_HEIGHT: u8 = 32,
        const DISPLAY_WIDTH:  u8 = 64,

        const MEMORY_SIZE_IN_BYTES:     u16 = 4096 * 8,
        const DISPLAY_SIZE:             u16 = DISPLAY_HEIGHT * DEISPLAY_WIDTH,

        const DELAY_TIMER_DECREMENT_RATE_IN_HZ: u8 = 60,

        let mut memory:             [u8, MEMORY_SIZE_IN_BYTES],
        let mut display:            [bool, DISPLAY_SIZE_IN_PIXELS],
        let mut program_counter:    u16,
        let mut index_register:     u16,
        let mut stack:              Vec::new(),
        let mut delay_timer:        u8,
        let mut sound_timer         u8,
        let mut registers           [u16, 0xF],
    }

    enum Opcodes {
        ClearScreen = 0x00E0,
        Jump    = 0x1,
        SetVX   = 0x6,
        AddToVX = 0x7,
        SetI    = 0xA,
        Draw    = 0xD,
    }
}




    
