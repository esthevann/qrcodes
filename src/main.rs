use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{ffi::OsString, path::Path};
use chrono::Utc;

use clap::Parser;
use image::Luma;

use qrcode::{QrCode, render::unicode};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = miette::Result<T, Error>;

// Generate Qr codes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// the value to be encoded
    string_to_encode: String,

    /// the path save the qrcode as png
    path_to_save: Option<PathBuf>,

    /// Print qr code to terminal as unicode
    #[arg(long)]
    print: bool,
}

fn main() -> Result<()> {
    let cli = Args::parse();

    let path = if let Some(path) = cli.path_to_save {
        path
    } else {
        format!("./{}.png", Utc::now().timestamp()).into()
    };

    encode_render_and_save(cli.string_to_encode, path, cli.print)?;
    Ok(())
}

fn encode_render_and_save<D: AsRef<[u8]>, P: AsRef<Path>>(
    data: D,
    path: P,
    print: bool,
) -> Result<()> {
    let code = QrCode::new(data)?;
    let image = code.render::<Luma<u8>>().build();
    image.save(path)?;

    if print {
        let image = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        println!("{image}");
    }
    Ok(())
}
