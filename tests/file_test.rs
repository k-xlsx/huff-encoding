use huff_encoding::file::*;



#[test]
fn compress_decompress() {
    let s = String::from("Mongo...
    a great barbarian from the north seeking to conquer new lands for his kingdom.
    Mysterio the Magnificent...
    a powerful wizard questing for the secret of immortality.");

    let encoded_bytes = compress(&s.as_bytes());
    let decoded_bytes = decompress(&encoded_bytes);
    assert_eq!(s, std::str::from_utf8(&decoded_bytes).unwrap());
    assert_eq!(s.as_bytes(), decoded_bytes);

    
    let encoded_bytes = threaded_compress(&s.as_bytes());
    let decoded_bytes = decompress(&encoded_bytes);
    assert_eq!(s, std::str::from_utf8(&decoded_bytes).unwrap());
    assert_eq!(s.as_bytes(), decoded_bytes);
}

#[test]
fn write_read_hfe() {
    let s = String::from("Erutan Revol...
    Elven Warden sworn to protect Nature, with his own life if need should arise
    Baron von Tarkin...
    Master of Death waging war against the forces of Life.");

    write_hfe("", "temp.hfe", &s.as_bytes()).expect("file write error");
    let decoded_bytes = read_hfe("temp.hfe").expect("file read error");
    assert_eq!(s, std::str::from_utf8(&decoded_bytes).unwrap());
    assert_eq!(s.as_bytes(), decoded_bytes);

    threaded_write_hfe("", "temp.hfe", &s.as_bytes()).expect("file threaded write error");
    let decoded_bytes = read_hfe("temp.hfe").expect("file read error");
    assert_eq!(s, std::str::from_utf8(&decoded_bytes).unwrap());
    assert_eq!(s.as_bytes(), decoded_bytes);

    std::fs::remove_file("temp.hfe").unwrap();
}