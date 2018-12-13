use crate::{gdt, hlt_loop, vga_buffer::Writer};
use core::fmt::Write;
use lazy_static::lazy_static;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

use pic8259_simple::ChainedPics;
use spin;

const PIC1_OFFSET: u8 = 32;
const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;
const TIMER_INTERRUPT_ID: u8 = PIC1_OFFSET;
const KEYBOARD_INTERRUPT_ID: u8 = PIC1_OFFSET + 1;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // set CPU exception handlers
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // set CPU interrupt handlers
        //idt[TIMER_INTERRUPT_ID as usize].set_handler_fn(timer_interrupt_handler);
        idt[KEYBOARD_INTERRUPT_ID as usize].set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

pub fn irq_set_mask(mut irq: u8) {
    use x86_64::instructions::port::Port;

    let mut pic_data_port: Port<u8> = if irq < 8 {
        Port::new(0x21)
    } else {
        irq -= 8;
        Port::new(0xa1)
    };

    unsafe {
        let mask = pic_data_port.read() | (1 << irq);
        pic_data_port.write(mask);
    }
}

pub fn irq_clear_mask(mut irq: u8) {
    use x86_64::instructions::port::Port;

    let mut pic_data_port: Port<u8> = if irq < 8 {
        Port::new(0x21)
    } else {
        irq -= 8;
        Port::new(0xa1)
    };

    unsafe {
        let mask = pic_data_port.read() & !(1 << irq);
        pic_data_port.write(mask);
    }
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut ExceptionStackFrame,
) {
    writeln!(Writer, "\n\nEXCEPTION: BREAKPOINT");
    writeln!(Writer, "{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    error_code: u64,
) {
    writeln!(Writer, "\n\nEXCEPTION: DOUBLE FAULT");
    writeln!(Writer, "{:#?}", stack_frame);
    writeln!(Writer, "ERROR_CODE: {:#x}", error_code);
    hlt_loop();
}

extern "x86-interrupt" fn timer_interrupt_handler(
    stack_frame: &mut ExceptionStackFrame,
) {
    write!(Writer, ".");
    unsafe { PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID) }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: &mut ExceptionStackFrame)
{
    use x86_64::instructions::port::Port;
    use pc_keyboard::{Keyboard, ScancodeSet1, DecodedKey, layouts};
    use spin::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1));
    }

    let mut keyboard = KEYBOARD.lock();
    let port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => write!(Writer, "{}", character),
                DecodedKey::RawKey(key) => write!(Writer, "{:?}", key),
            };
        }
    }

    unsafe { PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID) }
}
