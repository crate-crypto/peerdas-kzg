use common::collect_test_files;
use serde_::TestVector;
use std::fs;

mod common;

mod serde_ {
    use crate::common::UnsafeBytes;

    use super::common::bytes_from_hex;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct YamlInput {
        blob: String,
    }

    type YamlOutput = Vec<String>;

    #[derive(Deserialize)]
    struct YamlTestVector {
        input: YamlInput,
        output: Option<YamlOutput>,
    }

    pub struct TestVector {
        pub blob: UnsafeBytes,
        pub cells: Option<Vec<UnsafeBytes>>,
    }

    impl TestVector {
        pub fn from_str(yaml_data: &str) -> Self {
            let yaml_test_vector: YamlTestVector = serde_yaml::from_str(yaml_data).unwrap();
            TestVector::from(yaml_test_vector)
        }
    }

    impl From<YamlTestVector> for TestVector {
        fn from(yaml_test_vector: YamlTestVector) -> Self {
            let input = yaml_test_vector.input.blob;
            let output = yaml_test_vector.output;

            let input = bytes_from_hex(&input);

            let output = match output {
                Some(cells) => {
                    let cells: Vec<_> = cells.iter().map(|cell| bytes_from_hex(cell)).collect();
                    Some(cells)
                }
                None => None,
            };

            TestVector {
                blob: input,
                cells: output.map(|out| out),
            }
        }
    }
}

const TEST_DIR: &str = "../consensus_test_vectors/compute_cells";
#[test]
fn test_compute_cells() {
    let test_files = collect_test_files(TEST_DIR).unwrap();

    let prover_context = eip7594::prover::ProverContext::default();

    for test_file in test_files {
        let yaml_data = fs::read_to_string(test_file).unwrap();
        let test = TestVector::from_str(&yaml_data);

        let blob = match test.blob.try_into() {
            Ok(blob) => blob,
            Err(_) => {
                assert!(test.cells.is_none());
                continue;
            }
        };

        match prover_context.compute_cells(&blob) {
            Ok(cells) => {
                let expected_cells = test.cells.unwrap();

                for k in 0..expected_cells.len() {
                    let expected_cell = &expected_cells[k];

                    let got_cell = &cells[k];

                    assert_eq!(&got_cell[..], &expected_cell[..]);
                }
            }
            Err(_) => {
                // On an error, we expect the output to be null
                assert!(test.cells.is_none());
            }
        };
    }
}
