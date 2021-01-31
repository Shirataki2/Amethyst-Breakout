extern crate image;
use quick_xml::{Reader, events::{Event, BytesStart}};
use structopt::StructOpt;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufWriter, Write};
use image::GenericImageView;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt( parse(from_os_str))]
    input: PathBuf,
    #[structopt(short, long, default_value("png"))]
    image_ext: String,
}

fn main() {
    let opts = Opt::from_args();

    let input = opts.input;
    let mut image_input = input.clone();
    image_input.set_extension(opts.image_ext);

    let mut output_file = input.clone();
    output_file.set_extension("ron");
    let mut output_file = BufWriter::new(File::create(&output_file).expect("Failed to create output file"));

    writeln!(output_file, "List ((").unwrap();

    let (width, height) = image::open(image_input)
        .expect("Failed to open image")
        .dimensions();
    writeln!(output_file, "\ttexture_width: {},", width).unwrap();
    writeln!(output_file, "\ttexture_height: {},", height).unwrap();
    writeln!(output_file, "\tsprites: [").unwrap();

    let mut reader = Reader::from_file(input).expect("Failed to open xml file");
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => write_sprite_data(&mut output_file, e),
            Ok(Event::Eof) => break,
            Err(e) => panic!("Parse Error: {:?}", e),
            Ok(_) => {},
        }
        buf.clear();
    }
    writeln!(output_file, "\t]").unwrap();
    writeln!(output_file, "))").unwrap();
    println!("Success");
}

fn get_utf8(input: &[u8]) -> String {
    String::from_utf8(input.to_vec()).unwrap()
}

fn write_sprite_data<W: Write>(file: &mut BufWriter<W>, e: &BytesStart) {
    writeln!(file, "\t\t(").unwrap();
    e.attributes().for_each(|a| {
        let a = a.unwrap();
        if get_utf8(a.key) != "name".to_owned() {
            writeln!(file, "\t\t\t{}: {},", get_utf8(a.key), get_utf8(&a.value)).unwrap();
        }
    });
    writeln!(file, "\t\t),").unwrap();
}