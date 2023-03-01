fn main() {
    let mut printer = escpos::Model::TmT88v.printer();

    // printer.write("=================  BEGIN  ================\n".as_bytes()).unwrap();
    for i in 0..10 {
        printer.writeln("humor".as_bytes()).unwrap();
    }
    // printer.write("=================   END   ================\n".as_bytes()).unwrap();

    printer.flush().unwrap();

    printer.feed(5).unwrap();
    printer.cut().unwrap();
    printer.finish().unwrap();
}
