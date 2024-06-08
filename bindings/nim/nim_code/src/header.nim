# WARNING: This file has been automatically generated by nbindgen. Do not edit by hand.


const NUM_BYTES_PROOFS* = 6144

const NUM_BYTES_CELLS* = 262144


## The settings object for the Context.
# This is used to indicate if the context is for proving only, verifying only or both.
type CContextSetting* = enum
  ProvingOnly
  VerifyOnly
  Both

## A C-style enum to indicate the status of each function call
type CResultStatus* = enum
  Ok
  Err

type PeerDASContext* {.incompleteStruct.} = object

## The return value of each function call.
# This is a C-style struct that contains the status of the call and an error message, if
# the status was an error.
type CResult* = object
  xstatus*: CResultStatus
  xerror_msg*: pointer

proc peerdas_context_new*(): ptr PeerDASContext {.importc: "peerdas_context_new".}

proc peerdas_context_new_with_setting*(setting: CContextSetting): ptr PeerDASContext {.importc: "peerdas_context_new_with_setting".}

## Safety:
# - The caller must ensure that the pointer is valid. If the pointer is null, this method will return early.
# - The caller should also avoid a double-free by setting the pointer to null after calling this method.
proc peerdas_context_free*(ctx: ptr PeerDASContext): void {.importc: "peerdas_context_free".}

proc free_error_message*(c_message: pointer): void {.importc: "free_error_message".}

## Safety:
# - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error.
# - The caller must ensure that `blob` points to a region of memory that is at least `BYTES_PER_BLOB` bytes.
# - The caller must ensure that `out` points to a region of memory that is at least `BYTES_PER_COMMITMENT` bytes.
proc blob_to_kzg_commitment*(ctx: ptr PeerDASContext,
                             blob_length: uint64,
                             blob: pointer,
                             outx: pointer): CResult {.importc: "blob_to_kzg_commitment".}

## Safety:
# - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error.
# - The caller must ensure that `blob` points to a region of memory that is at least `BYTES_PER_BLOB` bytes.
# - The caller must ensure that `out_cells` points to a region of memory that is at least `NUM_BYTES_CELLS` bytes.
proc compute_cells*(ctx: ptr PeerDASContext,
                    blob_length: uint64,
                    blob: pointer,
                    out_cells: pointer): CResult {.importc: "compute_cells".}

## Safety:
# - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error.
# - The caller must ensure that `blob` points to a region of memory that is at least `BYTES_PER_BLOB` bytes.
# - The caller must ensure that `out_cells` points to a region of memory that is at least `NUM_BYTES_CELLS` bytes.
# - The caller must ensure that `out_proofs` points to a region of memory that is at least `NUM_BYTES_PROOFS` bytes.
proc compute_cells_and_kzg_proofs*(ctx: ptr PeerDASContext,
                                   blob_length: uint64,
                                   blob: pointer,
                                   out_cells: pointer,
                                   out_proofs: pointer): CResult {.importc: "compute_cells_and_kzg_proofs".}

## Safety:
# - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error.
# - The caller must ensure that `cell` points to a region of memory that is at least `BYTES_PER_CELL` bytes.
# - The caller must ensure that `commitment` points to a region of memory that is at least `BYTES_PER_COMMITMENT` bytes.
# - The caller must ensure that `proof` points to a region of memory that is at least `BYTES_PER_COMMITMENT` bytes.
# - The caller must ensure that `verified` points to a region of memory that is at least 1 byte.
proc verify_cell_kzg_proof*(ctx: ptr PeerDASContext,
                            cell_length: uint64,
                            cell: pointer,
                            commitment_length: uint64,
                            commitment: pointer,
                            cell_id: uint64,
                            proof_length: uint64,
                            proof: pointer,
                            verified: pointer): CResult {.importc: "verify_cell_kzg_proof".}

## Safety:
# - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error.
# - The caller must ensure that `row_commitments` points to a region of memory that is at least `row_commitments_length` bytes.
# - The caller must ensure that `row_indices` points to a region of memory that is at least `num_cells` bytes.
# - The caller must ensure that `column_indices` points to a region of memory that is at least `num_cells` bytes.
# - The caller must ensure that `cells` points to a region of memory that is at least `cells_length` bytes.
# - The caller must ensure that `proofs` points to a region of memory that is at least `num_cells * BYTES_PER_COMMITMENT` bytes.
# - The caller must ensure that `verified` points to a region of memory that is at least 1 byte.
#
# Note: cells, proofs and row_commitments are expected to be contiguous in memory.
# ie they have been concatenated together
proc verify_cell_kzg_proof_batch*(ctx: ptr PeerDASContext,
                                  row_commitments_length: uint64,
                                  row_commitments: pointer,
                                  row_indices_length: uint64,
                                  row_indices: pointer,
                                  column_indices_length: uint64,
                                  column_indices: pointer,
                                  cells_length: uint64,
                                  cells: pointer,
                                  proofs_length: uint64,
                                  proofs: pointer,
                                  verified: pointer): CResult {.importc: "verify_cell_kzg_proof_batch".}

## Safety:
# - The caller must ensure that the pointers are valid. If pointers are null, this method will return an error.
# - The caller must ensure that `cell_ids` points to a region of memory that is at least `num_cells` bytes.
# - The caller must ensure that `cells` points to a region of memory that is at least `cells_length` bytes.
# - The caller must ensure that `out_cells` points to a region of memory that is at least `NUM_BYTES_CELLS` bytes.
proc recover_all_cells*(ctx: ptr PeerDASContext,
                        cells_length: uint64,
                        cells: pointer,
                        cell_ids_length: uint64,
                        cell_ids: pointer,
                        out_cells: pointer): CResult {.importc: "recover_all_cells".}
