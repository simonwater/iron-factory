pub use storage_engine::kv_storage::KVStorage;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    kv_store FILE get KEY
    kv_store FILE delete KEY
    kv_store FILE insert KEY VALUE
    kv_store FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    kv_store FILE get KEY
    kv_store FILE delete KEY
    kv_store FILE insert KEY VALUE
    kv_store FILE update KEY VALUE
";

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let file_name = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(file_name);
    let mut store = KVStorage::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => match store.get(key) {
            Ok(None) => eprintln!("{:?} not found", key),
            Ok(Some(value)) => println!("{:?}", value),
            Err(err) => eprintln!("{}", err),
        },
        "delete" => {
            store.delete(key).unwrap();
        }
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap();
        }
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap();
        }
        _ => eprintln!("{}", &USAGE),
    }
}
