use anyhow::Result;
use wasmer::{imports, Instance, Module, RuntimeError, Store, Value};

pub fn execute_smart_contract(
    wasm_bytes: &[u8],
    function: &str,
    params: &[Value],
) -> Result<Box<[Value]>, RuntimeError> {
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes).map_err(|e| RuntimeError::new(&e.to_string()))?;
    let import_object = imports! {};
    let instance =
        Instance::new(&module, &import_object).map_err(|e| RuntimeError::new(&e.to_string()))?;
    let func = instance
        .exports
        .get_function(function)
        .map_err(|e| RuntimeError::new(&e.to_string()))?;
    func.call(params)
        .map_err(|e| RuntimeError::new(&e.to_string()))
}
