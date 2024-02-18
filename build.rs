use tonic_build::*;

fn main() {
    compile_protos("proto/license.proto")
        .unwrap();
}