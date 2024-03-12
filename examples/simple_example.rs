use httpp::ServeMux;
use httpp::Status;

fn main() {
    let mut mux = ServeMux::new();

    mux.handle_func("/hej", |w, _| {
        w.write_header(Status::Accepted);
        w.write(b"Hello World");
    });

    mux.handle_func("/", |w, _| {
        w.write_header(Status::OK);
        w.write(b"This is root");
    });

    println!("Serving on :8888");
    mux.listen_and_serve("127.0.0.1:8888")
}
