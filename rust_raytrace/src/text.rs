use super::fb::*;
use super::low_level::hartid;
use super::math::Color;

const BITMAP_FONT: [(u32, u32); 96] = [
    (0, 0),                   // 0  32 ' '
    (4472896, 4472896), // 1  33 '!' // 0100 0100 0100 0000 0100 0000      0100 0100 0100 0000 0100 0000
    (11141120, 11141120), // 2  34 '"' // 1010 1010 0000 0000 0000 0000      1010 1010 0000 0000 0000 0000
    (11447968, 11447968), // 3  35 '#' // 1010 1110 1010 1110 1010 0000      1010 1110 1010 1110 1010 0000
    (5162720, 5162724), // 4  36 '$' // 0100 1110 1100 0110 1110 0000      0100 1110 1100 0110 1110 0100
    (0, 0),             // 5  37 '%' // NOT WRITTEN
    (4868704, 15395552), // 6  38 '&' // 0100 1010 0100 1010 0110 0000      1110 1010 1110 1010 1110 0000
    (4456448, 4456448), // 7  39 ''' // 0100 0100 0000 0000 0000 0000      1110 1010 1110 1010 1110 0000
    (2376736, 6571104), // 8  40 '(' // 0010 0100 0100 0100 0010 0000      0110 0100 0100 0100 0110 0000
    (8668288, 12862656), // 9  41 ')' // 1000 0100 0100 0100 1000 0000      1100 0100 0100 0100 1100 0000
    (674304, 978432), // 10 42 '*' // 0000 1010 0100 1010 0000 0000      0000 1110 1110 1110 0000 0000
    (320512, 320512), // 11 43 '+' // 0000 0100 1110 0100 0000 0000      0000 0100 1110 0100 0000 0000
    (1088, 1228), // 12 44 ',' // 0000 0000 0000 0100 0100 0000      0000 0000 0000 0100 1100 1100
    (57344, 57344), // 13 45 '-' // 0000 0000 1110 0000 0000 0000      0000 0000 1110 0000 0000 0000
    (64, 64),     // 14 46 '.' // 0000 0000 0000 0000 0100 0000      0000 0000 0000 0000 0100 0000
    (2246784, 2287744), // 15 47 '/' // 0010 0010 0100 1000 1000 0000      0010 0010 1110 1000 1000 0000
    (6990528, 15379168), // 16 48 '0' // 0110 1010 1010 1010 1100 0000      1110 1010 1010 1010 1110 0000
    (4998368, 4998368), // 17 49 '1' // 0100 1100 0100 0100 1110 0000      0100 1100 0100 0100 1110 0000
    (14870752, 14870752), // 18 50 '2' // 1110 0010 1110 1000 1110 0000      1110 0010 1110 1000 1110 0000
    (14828256, 14836448), // 19 51 '3' // 1110 0010 0100 0010 1110 0000      1110 0010 0110 0010 1110 0000
    (9101856, 9101856), // 20 52 '4' // 1000 1010 1110 0010 0010 0000      1000 1010 1110 0010 0010 0000
    (15262432, 15262432), // 21 53 '5' // 1110 1000 1110 0010 1110 0000      1110 1000 1110 0010 1110 0000
    (6875872, 15264480), // 22 54 '6' // 0110 1000 1110 1010 1110 0000      1110 1000 1110 1010 1110 0000
    (14829120, 14836800), // 23 55 '7' // 1110 0010 0100 0110 0100 0000      1110 0010 0110 0100 0100 0000
    (15395552, 15395552), // 24 56 '8' // 1110 1010 1110 1010 1110 0000      1110 1010 1110 1010 1110 0000
    (15393472, 15393504), // 25 57 '9' // 1110 1010 1110 0010 1100 0000      1110 1010 1110 0010 1110 0000
    (263168, 263168), // 26 58 ':' // 0000 0100 0000 0100 0000 0000      0000 0100 0000 0100 0000 0000
    (263232, 263244), // 27 59 ';' // 0000 0100 0000 0100 0100 0000      0000 0100 0000 0100 0100 1100
    (2393120, 7261792), // 28 60 '<' // 0010 0100 1000 0100 0010 0000      0110 1110 1100 1110 0110 0000
    (921088, 921088), // 29 61 '=' // 0000 1110 0000 1110 0000 0000      0000 1110 0000 1110 0000 0000
    (8660096, 13528768), // 30 62 '>' // 1000 0100 0010 0100 1000 0000      1100 1110 0110 1110 1100 0000
    (12730432, 14836800), // 31 63 '?' // 1100 0010 0100 0000 0100 0000      1110 0010 0110 0100 0100 0000
    (0, 0),               // 32 64 '@' // NOT WRITTEN
    (15395488, 15395488), // 33 65 'A' // 1110 1010 1110 1010 1010 0000      1110 1010 1110 1010 1010 0000
    (15387360, 15395552), // 34 66 'B' // 1110 1010 1100 1010 1110 0000      1110 1010 1110 1010 1110 0000
    (15239392, 15239392), // 35 67 'C' // 1110 1000 1000 1000 1110 0000      1110 1000 1000 1000 1110 0000
    (13281984, 15379168), // 36 68 'D' // 1100 1010 1010 1010 1100 0000      1110 1010 1010 1010 1110 0000
    (15255776, 15255776), // 37 69 'E' // 1110 1000 1100 1000 1110 0000      1110 1000 1100 1000 1110 0000
    (15255680, 15255680), // 38 70 'F' // 1110 1000 1100 1000 1000 0000      1110 1000 1100 1000 1000 0000
    (15248096, 15248096), // 39 71 'G' // 1110 1000 1010 1010 1110 0000      1110 1000 1010 1010 1110 0000
    (11201184, 11201184), // 40 72 'H' // 1010 1010 1110 1010 1010 0000      1010 1010 1110 1010 1010 0000
    (14959840, 14959840), // 41 73 'I' // 1110 0100 0100 0100 1110 0000      1110 0100 0100 0100 1110 0000
    (2239200, 2239200), // 42 74 'J' // 0010 0010 0010 1010 1110 0000      0010 0010 0010 1010 1110 0000
    (11192992, 11201184), // 43 75 'K' // 1010 1010 1100 1010 1010 0000      1010 1010 1110 1010 1010 0000
    (8947936, 8947936), // 44 76 'L' // 1000 1000 1000 1000 1110 0000      1000 1000 1000 1000 1110 0000
    (11463328, 15657632), // 45 77 'M' // 1010 1110 1110 1010 1010 0000      1110 1110 1110 1010 1010 0000
    (13281952, 15379104), // 46 78 'N' // 1100 1010 1010 1010 1010 0000      1110 1010 1010 1010 1010 0000
    (15379168, 15379168), // 47 79 'O' // 1110 1010 1010 1010 1110 0000      1110 1010 1010 1010 1110 0000
    (15394944, 15394944), // 48 80 'P' // 1110 1010 1110 1000 1000 0000      1110 1010 1110 1000 1000 0000
    (15379040, 15379168), // 49 81 'Q' // 1110 1010 1010 1010 0110 0000      1110 1010 1010 1010 1110 0000
    (15387296, 15395488), // 50 82 'R' // 1110 1010 1100 1010 1010 0000      1110 1010 1110 1010 1010 0000
    (6873792, 15262432), // 51 83 'S' // 0110 1000 1110 0010 1100 0000      1110 1000 1110 0010 1110 0000
    (14959680, 14959680), // 52 84 'T' // 1110 0100 0100 0100 0100 0000      1110 0100 0100 0100 0100 0000
    (11184736, 11184864), // 53 85 'U' // 1010 1010 1010 1010 0110 0000      1010 1010 1010 1010 1110 0000
    (11445472, 11202112), // 54 86 'V' // 1010 1110 1010 0100 1110 0000      1010 1010 1110 1110 0100 0000
    (11202208, 11202272), // 55 87 'W' // 1010 1010 1110 1110 1010 0000      1010 1010 1110 1110 1110 0000
    (11160224, 11201184), // 56 88 'X' // 1010 1010 0100 1010 1010 0000      1010 1010 1110 1010 1010 0000
    (15352896, 11420736), // 57 89 'Y' // 1110 1010 0100 0100 0100 0000      1010 1110 0100 0100 0100 0000
    (14829792, 14870752), // 58 90 'Z' // 1110 0010 0100 1000 1110 0000      1110 0010 1110 1000 1110 0000
    (0, 0),               // 59 91 '[' // NOT WRITTEN
    (0, 0),               // 60 92 '\' // NOT WRITTEN
    (0, 0),               // 61 93 ']' // NOT WRITTEN
    (4849664, 15597568), // 62 94 '^' // 0100 1010 0000 0000 0000 0000      1110 1110 0000 0000 0000 0000
    (224, 224), // 63 95 '_' // 0000 0000 0000 0000 1110 0000      0000 0000 0000 0000 1110 0000
    (0, 0),     // 64 96 '`' // NOT WRITTEN
    (436832, 961248), // 65 97 'a' // 0000 0110 1010 1010 0110 0000      0000 1110 1010 1010 1110 0000
    (9349856, 9349856), // 66 98 'b' // 1000 1110 1010 1010 1110 0000      1000 1110 1010 1010 1110 0000
    (952544, 952544), // 67 99 'c' // 0000 1110 1000 1000 1110 0000      0000 1110 1000 1000 1110 0000
    (3058400, 3058400), // 68 100 'd' // 0010 1110 1010 1010 1110 0000      0010 1110 1010 1010 1110 0000
    (961760, 962272), // 69 101 'e' // 0000 1110 1010 1100 1110 0000      0000 1110 1010 1110 1110 0000
    (6612032, 6612032), // 70 102 'f' // 0110 0100 1110 0100 0100 0000      0110 0100 1110 0100 0100 0000
    (976608, 962272), // 71 103 'g' // 0000 1110 1110 0110 1110 0000      0000 1110 1010 1110 1110 0000
    (9349792, 9349792), // 72 104 'h' // 1000 1110 1010 1010 1010 0000      1000 1110 1010 1010 1010 0000
    (4474080, 4867296), // 73 105 'i' // 0100 0100 0100 0100 1110 0000      0100 1010 0100 0100 1110 0000
    (2239200, 2435808), // 74 106 'j' // 0010 0010 0010 1010 1110 0000      0010 0101 0010 1010 1110 0000
    (9096352, 9105056), // 75 107 'k' // 1000 1010 1100 1100 1010 0000      1000 1010 1110 1110 1010 0000
    (4474080, 12862688), // 76 108 'l' // 0100 0100 0100 0100 1110 0000      1100 0100 0100 0100 1110 0000
    (715424, 977568), // 77 109 'm' // 0000 1010 1110 1010 1010 0000      0000 1110 1110 1010 1010 0000
    (830112, 961184), // 78 110 'n' // 0000 1100 1010 1010 1010 0000      0000 1110 1010 1010 1010 0000
    (961248, 961248), // 79 111 'o' // 0000 1110 1010 1010 1110 0000      0000 1110 1010 1010 1110 0000
    (962176, 962176), // 80 112 'p' // 0000 1110 1010 1110 1000 0000      0000 1110 1010 1110 1000 0000
    (962080, 962080), // 81 113 'q' // 0000 1110 1010 1110 0010 0000      0000 1110 1010 1110 0010 0000
    (714880, 968832), // 82 114 'r' // 0000 1010 1110 1000 1000 0000      0000 1110 1100 1000 1000 0000
    (968416, 968416), // 83 115 's' // 0000 1110 1100 0110 1110 0000      0000 1110 1100 0110 1110 0000
    (5129280, 5129280), // 84 116 't' // 0100 1110 0100 0100 0100 0000      0100 1110 0100 0100 0100 0000
    (699104, 699104), // 85 117 'u' // 0000 1010 1010 1010 1110 0000      0000 1010 1010 1010 1110 0000
    (715328, 700128), // 86 118 'v' // 0000 1010 1110 1010 0100 0000      0000 1010 1010 1110 1110 0000
    (700064, 700128), // 87 119 'w' // 0000 1010 1010 1110 1010 0000      0000 1010 1010 1110 1110 0000
    (672928, 716448), // 88 120 'x' // 0000 1010 0100 0100 1010 0000      0000 1010 1110 1110 1010 0000
    (713312, 713440), // 89 121 'y' // 0000 1010 1110 0010 0110 0000      0000 1010 1110 0010 1110 0000
    (945376, 945376), // 90 122 'z' // 0000 1110 0110 1100 1110 0000      0000 1110 0110 1100 1110 0000
    (0, 0),           // 91 123 '(' // NOT WRITTEN
    (0, 0),           // 92 124 '|' // NOT WRITTEN
    (0, 0),           // 93 125 ')' // NOT WRITTEN
    (0, 0),           // 94 126 '~' // NOT WRITTEN
    (0xffffffff, 0xffffffff), // 95 127 ERROR SQUARE
];

fn draw_char(ch: u8, px: u32, py: u32, scale: u32, col: &Color) {
    let ch = if ch == 0 { 127 } else { ch };
    if ch < 32 || ch > 127 {
        return;
    }
    let bitmap_a: u32 = BITMAP_FONT[(ch - 32) as usize].0;
    let bitmap_b: u32 = BITMAP_FONT[(ch - 32) as usize].1;
    if scale <= 2 {
        for x in 0..(4 * scale) {
            for y in 0..(6 * scale) {
                if ((bitmap_a >> (5 - (y / scale)) * 4) >> (4 - (x / scale))) & 1 == 1 {
                    if ((bitmap_b >> (5 - (y / scale)) * 4) >> (4 - (x / scale))) & 1 == 1 {
                        write_px_raw(px + x, py + y, col.to_u128());
                    }
                }
            }
        }
    } else {
        for x in 0..4 {
            for y in 0..6 {
                if ((bitmap_a >> (5 - y) * 4) >> (4 - x)) & 1 == 1 {
                    if ((bitmap_b >> (5 - y) * 4) >> (4 - x)) & 1 == 1 {
                        unsafe {
                            write_px_memop(
                                px + x * scale,
                                py + y * scale,
                                col.clone(),
                                scale,
                            );
                        }
                    }
                }
            }
        }
    }
}

fn draw_line<S: AsRef<str>>(s: S, px: u32, py: u32, scale: u32, col: &Color) {
    let s = s.as_ref();
    let mut i = 0;
    for c in s.bytes() {
        draw_char(c, px + i * 5 * scale, py, scale, &col);
        i += 1;
    }
}

// defined by linker script
extern "C" {
    static _end_textbuf: usize;
}

/// put a character into the buffer for the output hart to read and print
unsafe fn print_char_unsafe(ch: u8) {
    // calculate text buffer base
    let end_textbuf = &_end_textbuf as *const usize as usize;
    let tbase = (end_textbuf - hartid() * 4096) as *mut usize;
    let cur_ptr = tbase.read_volatile();
    if cur_ptr == 4096 {
        // TODO: make work lol
        return;
    }
    // this works because while write visibility is not synchronized, the order
    // in which writes appear is always upheld
    ((cur_ptr + tbase as usize + 4) as *mut u8).write_unaligned(ch);
    tbase.write_volatile(cur_ptr + 1);
}

pub fn print_char(ch: u8) {
    unsafe {
        print_char_unsafe(ch);
    }
}

const STARTY: u32 = 1622;
const HEADER: &str = "rvc (RISC-V CPU emulator) by _pi_";
const HEADER_REPO: &str = "github.com/pimaker/rvc";
const HEADER_STATE: &str = "above: main memory (which doubles as framebuffer)";
const HEADER_TEXT: &str = "below: hart nr. 8 collects text output from others and draws it";
const WHITE: Color = Color { r: 255, g: 255, b: 255 };
const GRAY: Color = Color { r: 120, g: 120, b: 120 };
const PURPLE: Color = Color { r: 255, g: 40, b: 255 };
pub fn render_text() -> ! {
    draw_line(HEADER, 1024, STARTY, 6, &PURPLE);
    draw_line(HEADER_REPO, 1024, STARTY + 36, 3, &GRAY);
    draw_line(HEADER_STATE, 16, STARTY, 3, &WHITE);
    draw_line(HEADER_TEXT, 16, STARTY + 26, 3, &WHITE);
    draw_line("(hart = core in risc-v)", 138, STARTY + 32, 1, &GRAY);

    let mut ptrs = [0usize; 8];

    // if I make this an array of (0, 0) tuples it doesn't work - bug somewhere?
    let mut pos_x = [0; 8];
    let mut pos_y = [0; 8];

    loop {
        for hart in (0..8).rev() {
            unsafe {
                let end_textbuf = &_end_textbuf as *const usize as usize;
                let tbase = (end_textbuf - hart * 4096) as *const usize;
                let cur_ptr = tbase.read();
                while ptrs[hart] < cur_ptr {
                    let ch = ((ptrs[hart] + tbase as usize + 4) as *const u8).read_unaligned();
                    // if ch != 0 {
                    //     super::low_level::uart::print_char(ch);
                    // } else {
                    //     super::low_level::uart::print_char('0' as u8);
                    // }
                    ptrs[hart] += 1;
                    if ch == ('\n' as u8) {
                        pos_x[hart] = 0;
                        pos_y[hart] += 1;
                    } else {
                        draw_char(
                            ch,
                            pos_x[hart] * 5 + hart as u32 * 48 * 5 + 16,
                            pos_y[hart] * 7 + 64 + STARTY,
                            1,
                            if hart == 7 { &WHITE } else { &GRAY },
                        );
                        pos_x[hart] += 1;
                        if pos_x[hart] >= 48 {
                            pos_x[hart] = 0;
                            pos_y[hart] += 1;
                        }
                    }
                }
            }
        }

        // stall, we know that without a new commit cycle no new data will come
        unsafe {
            asm! { "fence.i" }
        }
    }
}