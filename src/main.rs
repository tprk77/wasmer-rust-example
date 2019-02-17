extern crate wasmer_runtime;

use std::str;

use wasmer_runtime::{
    imports,
    func,
    compile,
    error,
    Ctx,
    memory::MemoryView,
};

// Make sure that the compiled wasm-sample-app is accessible at this path.
static WASM: &'static [u8] = include_bytes!("../wasm-sample-app/target/wasm32-unknown-unknown/release/wasm_sample_app.wasm");

fn main() -> error::Result<()> {

    let module = compile(WASM)?;

    // Let's define the import object used to import our function
    // into our webassembly sample application.
    //
    // We've defined a macro that makes it super easy.
    //
    // The signature tells the runtime what the signature (the parameter
    // and return types) of the function we're defining here is.
    // The allowed types are `i32`, `u32`, `i64`, `u64`,
    // `f32`, and `f64`.
    //
    // Make sure to check this carefully!
    let import_object = imports! {
        // Define the "env" namespace that was implicitly used
        // by our sample application.
        "env" => {
            // name         // func    // signature
            "print_str" => func!(print_str), //print_str<[u32, u32] -> []>,
        },
    };

    // Compile our webassembly into an `Instance`.
    let instance = module.instantiate(&import_object)?;

    // Call our exported function!
    instance.call("hello_wasm", &[])?;

    Ok(())
}

// Let's define our "print_str" function.
fn print_str(ctx: &mut Ctx, ptr: u32, len: u32) {
    // Get a slice that maps to the memory currently used by the webassembly
    // instance.
    //
    // Webassembly only supports a single memory for now,
    // but in the near future, it'll support multiple.
    //
    // Therefore, we don't assume you always just want to access first
    // memory and force you to specify the first memory.
    let memory = ctx.memory(0);

    // Get a subslice that corresponds to the memory used by the string.
    let view: MemoryView<u8> = memory.view();
    let str_vec: Vec<u8> = view[ptr as usize..(ptr + len) as usize]
        .iter().map(|cell| cell.get()).collect();

    // Convert the subslice to a `&str`.
    let string = str::from_utf8(&str_vec).unwrap();

    // Print it!
    println!("{}", string);
}
