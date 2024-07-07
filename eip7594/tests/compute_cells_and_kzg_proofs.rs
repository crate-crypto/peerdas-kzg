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

    type YamlOutput = (Vec<String>, Vec<String>);

    #[derive(Deserialize)]
    struct YamlTestVector {
        input: YamlInput,
        output: Option<YamlOutput>,
    }

    #[derive(Debug, Clone)]
    pub struct KZGProofsAndCells {
        pub proofs: Vec<UnsafeBytes>,
        pub cells: Vec<UnsafeBytes>,
    }

    pub struct TestVector {
        pub blob: UnsafeBytes,
        pub proofs_and_cells: Option<KZGProofsAndCells>,
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
                Some((cells, kzg_proofs)) => {
                    let kzg_proofs: Vec<_> = kzg_proofs
                        .iter()
                        .map(|proof| bytes_from_hex(proof))
                        .collect();
                    let cells: Vec<_> = cells.iter().map(|cell| bytes_from_hex(cell)).collect();
                    Some((kzg_proofs, cells))
                }
                None => None,
            };

            TestVector {
                blob: input,
                proofs_and_cells: output.map(|out| KZGProofsAndCells {
                    proofs: out.0,
                    cells: out.1,
                }),
            }
        }
    }
}

const TEST_DIR: &str = "../consensus_test_vectors/compute_cells_and_kzg_proofs";
#[test]
fn test_compute_cells_and_kzg_proofs() {
    let test_files = collect_test_files(TEST_DIR).unwrap();

    let ctx = eip7594::PeerDASContext::default();

    for test_file in test_files {
        let yaml_data = fs::read_to_string(test_file).unwrap();
        let test = TestVector::from_str(&yaml_data);

        let blob = match test.blob.try_into() {
            Ok(blob) => blob,
            Err(_) => {
                assert!(test.proofs_and_cells.is_none());
                continue;
            }
        };

        match ctx.compute_cells_and_kzg_proofs(&blob) {
            Ok((cells, proofs)) => {
                let expected_proofs_and_cells = test.proofs_and_cells.unwrap();

                let expected_proofs = expected_proofs_and_cells.proofs;
                let expected_cells = expected_proofs_and_cells.cells;

                for k in 0..expected_proofs.len() {
                    let expected_proof = &expected_proofs[k];
                    let expected_cell = &expected_cells[k];

                    let got_proof = &proofs[k];
                    let got_cell = &cells[k];

                    assert_eq!(&got_cell[..], expected_cell);
                    assert_eq!(&got_proof[..], expected_proof);
                }
            }
            Err(_) => {
                // On an error, we expect the output to be null
                assert!(test.proofs_and_cells.is_none());
            }
        };
    }
}
