extern crate flatbuffers;

mod myschema_generated;

use flatbuffers::FlatBufferBuilder;
use myschema_generated::users::{finish_user_buffer, get_root_as_user, User, UserArgs};

fn main() {
    let mut bldr = FlatBufferBuilder::new();
    let mut bytes: Vec<u8> = Vec::new();

    make_user(&mut bldr, &mut bytes, "V", 1);

    let (name, id) = read_user(&bytes);
    println!(
        "{} has id {}. The encoded data is {} bytes long.",
        name,
        id,
        bytes.len()
    );
}

pub fn make_user(bldr: &mut FlatBufferBuilder, dest: &mut Vec<u8>, name: &str, id: u64) {
    // reset the `bytes` Vec to a clean state
    dest.clear();

    // reset the flatbuffers
    bldr.reset();

    //
    let args = UserArgs {
        name: Some(bldr.create_string(name)),
        id: id,
    };

    let user_offset = User::create(bldr, &args);

    finish_user_buffer(bldr, user_offset);

    let finished_data = bldr.finished_data();
    dest.extend_from_slice(finished_data);
}

pub fn read_user(buf: &[u8]) -> (&str, u64) {
    let u = get_root_as_user(buf);
    let name = u.name().unwrap();
    let id = u.id();
    (name, id)
}
