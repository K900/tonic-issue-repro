fn main() {
    tonic_build::configure()
        .format(false)
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();
}
