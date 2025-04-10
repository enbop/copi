use defmt::info;
use embedded_alloc::LlffHeap as Heap;

extern crate alloc;

const HEAP_SIZE: usize = 128 * 1024;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn init() {
    {
        use core::mem::MaybeUninit;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(core::ptr::addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }
    let mut engine = rhai::Engine::new();

    engine.on_print(|x| info!("[rhai] {}", x));

    info!("test rhai");
    let script = "print(40 + 2);";
    let ast = engine.compile(script).unwrap();
    engine.run_ast(&ast).unwrap();
    info!("run rhai");

    let result = engine.eval::<i32>("40 + 2").unwrap();
    info!("rhai eval result: {}", result);
}
