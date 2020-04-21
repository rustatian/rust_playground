use async_ssh::Session;
use futures::Future;

fn main() {
    let key = thrussh_keys::load_secret_key("", Some(b"")).unwrap();

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let ls_out = tokio_core::net::TcpStream::connect(&"".parse().unwrap(), &handle)
        .map_err(thrussh::Error::IO)
        .map_err(thrussh::HandlerError::Error)
        .and_then(|c| Session::new(c, &handle))
        .and_then(|session| session.authenticate_key("", key))
        .and_then(|mut session| session.open_exec("ls -la"));

    let channel = core.run(ls_out).unwrap();
    let (channel, data) = core.run(tokio_io::io::read_to_end(channel, Vec::new())).unwrap();
    let status = core.run(channel.exit_status()).unwrap();

    println!("{}", ::std::str::from_utf8(&data[..]).unwrap());
    println!("exited with: {}", status);
}
