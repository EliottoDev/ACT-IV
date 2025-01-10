use crate::library::global_config;

pub(crate) fn sync() {
    let r = global_config::write_library();
    println!("{:?}", r);
}
