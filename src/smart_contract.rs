use wasmer::{Store, Module, Instance, Value, imports};

pub fn execute_smart_contract(wasm_bytes: &[u8], function: &str, params: &[Value]) -> Result<Vec<Value>, String> {
    let mut store = Store::default();
    let module = Module::new(&store, wasm_bytes).map_err(|e| e.to_string())?;
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object).map_err(|e| e.to_string())?;
    let func = instance.exports.get_function(function).map_err(|e| e.to_string())?;
    let result = func.call(&mut store, params).map_err(|e| e.to_string())?;
    Ok(result.to_vec())
}
