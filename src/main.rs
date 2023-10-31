// https://os.phil-opp.com/ // using offsetpagetables
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(os_rust_project::test_runner)]
#![reexport_test_harness_main = "test_main"]
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os_rust_project::println;
use x86_64::structures::paging::PageTable;

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os_rust_project::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_rust_project::test_panic_handler(info)
}

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use os_rust_project::memory;
    use os_rust_project::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");
    os_rust_project::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    os_rust_project::hlt_loop();
}
