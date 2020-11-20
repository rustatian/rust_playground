use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

const PATH: &str = "foo.txt";

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create(PATH).await?;
    let _ = file.write(b"123123123hey").await?;


    let mut f = File::open(PATH).await?;
    let mut buffer = [0; 12];

    let _ = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..]);
    drop(f);

    let mut f = File::open(PATH).await?;

    let mut buff_vec = vec![];

    let _ = f.read_to_end(&mut buff_vec).await?;

    println!("The bytes: {:?}", buff_vec);

    Ok(())
}