#[allow(unused_imports)]
use asic_rs_core::data::device::HashAlgorithm;
use asic_rs_macros::ModelAlgorithm;

#[derive(ModelAlgorithm)]
enum Model {
    #[algorithm(HashAlgorithm::NotAnAlgorithm)]
    Invalid,
}

fn main() {}
