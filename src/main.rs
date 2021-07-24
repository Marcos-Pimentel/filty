mod error;
mod my_image;

use my_image::MyRgbImage;
use std::time::Instant;

pub fn main() -> crate::error::Result<()> {
    let neko_image = image::open("./neko.jpg")
        .expect("Erro ao abrir imagem")
        .to_rgb8();
    let tempo = Instant::now();
    let mut my_image = MyRgbImage::new(neko_image);
    my_image.mess_everything();
    my_image.save_image("./new_neko.jpg")?;
    println!("saca só essa execução: {:?}", tempo.elapsed());

    Ok(())
}
