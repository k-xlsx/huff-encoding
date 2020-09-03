use huff_encoding::huff_structs;
use huff_encoding::file_io;



fn main(){
    let s = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque eget venenatis lectus, eget pharetra massa. Etiam massa justo, varius vitae fermentum ac, finibus non ante. Praesent quis nunc erat. Suspendisse a euismod nisl. Nulla hendrerit, lacus vel venenatis tristique, nibh mauris tempus nunc, quis suscipit dolor sem at lorem. Donec et metus viverra leo faucibus imperdiet. Aenean pharetra porta est ac gravida. In eu aliquam ligula. Suspendisse potenti. Nulla facilisi. Fusce dictum nunc molestie ipsum pretium mollis. Cras hendrerit odio sit amet ante rutrum, nec rhoncus est volutpat.

    Duis tincidunt lorem hendrerit tempor dapibus. Sed eu ante sollicitudin, tristique diam sed, iaculis elit. Vivamus metus sem, finibus vel laoreet varius, ornare ac ante. Cras ac placerat massa. Vestibulum at dui purus. Suspendisse sagittis metus a metus suscipit sollicitudin. Nunc cursus risus vitae nibh pretium auctor. Nam cursus at leo eget dictum. Morbi scelerisque sagittis ultricies. Nunc placerat, neque vitae posuere accumsan, tellus ipsum condimentum nulla, sit amet sagittis diam elit mollis leo. Mauris blandit odio sit amet felis tempor porttitor. Donec sed gravida lorem, at tempor nunc. Maecenas vitae suscipit libero. Quisque ac elit tellus.
    
    Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Praesent gravida gravida leo, faucibus condimentum lorem rutrum et. Nulla pharetra gravida tempus. Donec ultrices ipsum eget interdum dignissim. Donec egestas iaculis massa sed luctus. Vivamus vestibulum tempor velit sed maximus. Integer ut vehicula risus. Aliquam at enim sapien. Proin molestie ac diam eu sodales. Sed elementum imperdiet orci vitae malesuada. Vestibulum sed quam convallis, sagittis lacus sit amet, laoreet ipsum. Cras scelerisque blandit lorem, eu consequat lorem tempus ut.
    
    Nam non diam laoreet, gravida nulla interdum, tempor sapien. Curabitur non gravida tellus. Etiam maximus, ex in volutpat tincidunt, nunc ante semper nisl, quis laoreet felis leo interdum nisi. Integer ut fermentum lacus, ut euismod urna. Sed quis enim sagittis, maximus mauris sollicitudin, blandit velit. Vestibulum id felis vel ligula convallis imperdiet. In vehicula dolor in lacus tincidunt, ac pretium tortor sodales. In accumsan, magna ut lobortis accumsan, quam lacus mattis tortor, in molestie turpis ligula nec arcu. Etiam ultrices et dui eu gravida. Nulla malesuada lorem eu hendrerit aliquet. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam quis ullamcorper lacus. Suspendisse ullamcorper malesuada ex, in dictum lorem mattis eget. Quisque eu bibendum nisi. Quisque dapibus libero lobortis tellus cursus auctor. Suspendisse potenti.
    
    Nullam placerat imperdiet dui, et pulvinar erat venenatis iaculis. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum aliquam dolor augue, id elementum nibh volutpat non. Donec id odio augue. Maecenas aliquam, sapien eget finibus luctus, arcu urna aliquet libero, ac tincidunt nibh lectus eu erat. Vivamus pharetra nisl sit amet neque commodo sollicitudin. Aenean efficitur pulvinar risus quis faucibus. Mauris sit amet fringilla quam. Etiam vel bibendum dolor, sit amet tempus felis.";

    
    let tree = huff_structs::HuffTree::from(s);

    let h = file_io::get_header(&mut tree.as_bin());
    let es = file_io::get_encoded_string(s, tree.char_codes());
    println!("{:?}\n", h);
    println!("{:?}", es);
    println!("{:#?}", tree.char_codes())
}
