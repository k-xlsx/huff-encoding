use structopt::StructOpt;

mod cli;

// TODO: TESTS

fn main(){
    let start = std::time::Instant::now();
    println!("START");
    //---------------------------\\



    let s = String::from("Hello, World!aaąŁ");

    huff_encoding::write_hfe("", "temp.hfe", &s.as_bytes()).expect("file write error");
    let decoded_bytes = huff_encoding::read_hfe("temp.hfe").expect("file read error");

    println!("{:?}", s);
    println!("{:?}", s.as_bytes());
    println!("{:?}", std::str::from_utf8(&decoded_bytes));
    println!("{:?}\n", decoded_bytes);

    //......................\\

    let s = String::from("Hello, World!aaąŁ");

    huff_encoding::threaded_write_hfe("", "temp.hfe", &s.as_bytes()).expect("file write error");
    let decoded_bytes = huff_encoding::read_hfe("temp.hfe").expect("file read error");

    println!("{:?}", s);
    println!("{:?}", s.as_bytes());
    println!("{:?}", std::str::from_utf8(&decoded_bytes));
    println!("{:?}\n", decoded_bytes);

    //......................\\

    let s = String::from("Hello, World!aaąŁ");
    
    let encoded_bytes = huff_encoding::compress(&s.as_bytes());
    let decoded_bytes = huff_encoding::decompress(&encoded_bytes);

    println!("{:?}", s);
    println!("{:?}", s.as_bytes());
    println!("{:?}", std::str::from_utf8(&decoded_bytes));
    println!("{:?}\n", decoded_bytes);

    //......................\\

    let s = String::from("Hello, World!aaąŁ");
    
    let encoded_bytes = huff_encoding::threaded_compress(&s.as_bytes());
    let decoded_bytes = huff_encoding::decompress(&encoded_bytes);

    println!("{:?}", s);
    println!("{:?}", s.as_bytes());
    println!("{:?}", std::str::from_utf8(&decoded_bytes));
    println!("{:?}\n", decoded_bytes);


    
    //---------------------------\\
    let elapsed = start.elapsed();
    println!("{:?}\nEND", elapsed);
}
