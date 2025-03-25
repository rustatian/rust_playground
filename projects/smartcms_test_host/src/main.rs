use std::io::Read;

wasmtime::component::bindgen!({
    path: "./smart_cms.wit",
    world: "app",
});

struct KeyValue {
    mem: std::collections::HashMap<String, String>,
}

impl component::smartcms::kvstore::Host for KeyValue {
    fn get(&mut self, key: String) -> String {
        self.mem.get(&key).cloned().unwrap()
    }

    fn set(&mut self, key: String, value: String) {
        self.mem.insert(key, value);
    }
}

struct State {
    key_value: KeyValue,
}

fn main() {
    let mut config = wasmtime::Config::default();
    config.wasm_component_model(true);
    let engine = wasmtime::Engine::new(&config).unwrap();

    let mut store = wasmtime::Store::new(
        &engine,
        State {
            key_value: KeyValue {
                mem: std::collections::HashMap::new(),
            },
        },
    );

    // read the guest.wasm file
    let mut file = std::fs::File::open("guest.wasm").unwrap();
    let mut buff = Vec::new();
    file.read_to_end(&mut buff).unwrap();

    let component = wasmtime::component::Component::new(&engine, buff).unwrap();
    let mut linker = wasmtime::component::Linker::new(&engine);
    component::smartcms::kvstore::add_to_linker(&mut linker, |state: &mut State| {
        &mut state.key_value
    })
    .unwrap();

    let instance = App::instantiate(&mut store, &component, &linker).unwrap();
    println!("{:?}", instance.call_run(&mut store).unwrap());
}
