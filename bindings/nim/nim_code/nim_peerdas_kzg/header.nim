# WARNING: This file has been automatically generated by nbindgen. Do not edit by hand.



## A C-style enum to indicate whether a function call was a success or not.
type CResultStatus* = enum
  Ok
  Err

type PeerDASContext* {.incompleteStruct.} = object

## A C-style struct to represent the success result of a function call.
#
# This includes the status of the call and an error message, if the status was an error.
type CResult* = object
  xstatus*: CResultStatus
  xerror_msg*: pointer

## Create a new PeerDASContext and return a pointer to it.
#
# # Memory faults
#
# To avoid memory leaks, one should ensure that the pointer is freed after use
# by calling `peerdas_context_free`.
proc peerdas_context_new*(): ptr PeerDASContext {.importc: "peerdas_context_new".}

## # Safety
#
# - The caller must ensure that the pointer is valid. If the pointer is null, this method will return early.
# - The caller should also avoid a double-free by setting the pointer to null after calling this method.
#
# # Memory faults
#
# - If this method is called twice on the same pointer, it will result in a double-free.
#
# # Undefined behavior
#
# - Since the `ctx` is created in Rust, we can only get undefined behavior, if the caller passes in
# a pointer that was not created by `peerdas_context_new`.
proc peerdas_context_free*(ctx: ptr PeerDASContext): void {.importc: "peerdas_context_free".}

## Free the memory allocated for the error message.
#
# # Safety
#
# - The caller must ensure that the pointer is valid. If the pointer is null, this method will return early.
# - The caller should also avoid a double-free by setting the pointer to null after calling this method.
proc free_error_message*(c_message: pointer): void {.importc: "free_error_message".}

## Compute a commitment from a Blob
#
# # Safety
#
# - The caller must ensure that the pointers are valid.
# - The caller must ensure that `blob` points to a region of memory that is at least `BYTES_PER_BLOB` bytes.
# - The caller must ensure that `out` points to a region of memory that is at least `BYTES_PER_COMMITMENT` bytes.
#
# # Undefined behavior
#
# - This implementation will check if the ctx pointer is null, but it will not check if the other arguments are null.
#   If the other arguments are null, this method will dereference a null pointer and result in undefined behavior.
proc blob_to_kzg_commitment*(ctx: ptr PeerDASContext,
                             blob: pointer,
                             outx: pointer): CResult {.importc: "blob_to_kzg_commitment".}

## Computes the cells and KZG proofs for a given blob.
#
# # Safety
#
# - The caller must ensure that the pointers are valid. If pointers are null.
# - The caller must ensure that `blob` points to a region of memory that is at least `BYTES_PER_BLOB` bytes.
# - The caller must ensure that `out_cells` points to a region of memory that is at least `CELLS_PER_EXT_BLOB` elements
#   and that each element is at least `BYTES_PER_CELL` bytes.
# - The caller must ensure that `out_proofs` points to a region of memory that is at least `CELLS_PER_EXT_BLOB` elements
#   and that each element is at least `BYTES_PER_COMMITMENT` bytes.
#
# # Undefined behavior
#
# - This implementation will check if the ctx pointer is null, but it will not check if the other arguments are null.
#   If the other arguments are null, this method will dereference a null pointer and result in undefined behavior.
proc compute_cells_and_kzg_proofs*(ctx: ptr PeerDASContext,
                                   blob: pointer,
                                   out_cells: ptr pointer,
                                   out_proofs: ptr pointer): CResult {.importc: "compute_cells_and_kzg_proofs".}

## Verifies a batch of cells and their KZG proofs.
#
# # Safety
#
# - If the length parameter for a pointer is set to zero, then this implementation will not check if its pointer is
#   null. This is because the caller might have passed in a null pointer, if the length is zero. Instead an empty slice
#   will be created.
#
# - The caller must ensure that the pointers are valid.
# - The caller must ensure that `row_commitments` points to a region of memory that is at least `row_commitments_length` commitments
#   and that each commitment is at least `BYTES_PER_COMMITMENT` bytes.
# - The caller must ensure that `row_indices` points to a region of memory that is at least `num_cells` elements
#   and that each element is 8 bytes.
# - The caller must ensure that `column_indices` points to a region of memory that is at least `num_cells` elements
#   and that each element is 8 bytes.
# - The caller must ensure that `cells` points to a region of memory that is at least `cells_length` proof and
#   that each cell is at least `BYTES_PER_CELL` bytes
# - The caller must ensure that `proofs` points to a region of memory that is at least `proofs_length` proofs
#    and that each proof is at least `BYTES_PER_COMMITMENT` bytes.
# - The caller must ensure that `verified` points to a region of memory that is at least 1 byte.
#
# # Undefined behavior
#
# - This implementation will check if the ctx pointer is null, but it will not check if the other arguments are null.
#   If the other arguments are null, this method will dereference a null pointer and result in undefined behavior.
proc verify_cell_kzg_proof_batch*(ctx: ptr PeerDASContext,
                                  row_commitments_length: uint64,
                                  row_commitments: ptr pointer,
                                  row_indices_length: uint64,
                                  row_indices: pointer,
                                  column_indices_length: uint64,
                                  column_indices: pointer,
                                  cells_length: uint64,
                                  cells: ptr pointer,
                                  proofs_length: uint64,
                                  proofs: ptr pointer,
                                  verified: pointer): CResult {.importc: "verify_cell_kzg_proof_batch".}

## Recovers all cells and their KZG proofs from the given cell indices and cells
#
# # Safety
#
#  - If the length parameter for a pointer is set to zero, then this implementation will not check if its pointer is
#   null. This is because the caller might have passed in a null pointer, if the length is zero. Instead an empty slice
#   will be created.
#
# - The caller must ensure that the pointers are valid.
# - The caller must ensure that `cells` points to a region of memory that is at least `cells_length` cells
#   and that each cell is at least `BYTES_PER_CELL` bytes.
# - The caller must ensure that `cell_indices` points to a region of memory that is at least `cell_indices_length` cell indices
#   and that each cell id is 8 bytes.
# - The caller must ensure that `out_cells` points to a region of memory that is at least `CELLS_PER_EXT_BLOB` cells
#   and that each cell is at least `BYTES_PER_CELL` bytes.
# - The caller must ensure that `out_proofs` points to a region of memory that is at least `CELLS_PER_EXT_BLOB` proofs
#   and that each proof is at least `BYTES_PER_COMMITMENT` bytes.
#
# # Undefined behavior
#
# - This implementation will check if the ctx pointer is null, but it will not check if the other arguments are null.
#   If the other arguments are null, this method will dereference a null pointer and result in undefined behavior.
proc recover_cells_and_proofs*(ctx: ptr PeerDASContext,
                               cells_length: uint64,
                               cells: ptr pointer,
                               cell_indices_length: uint64,
                               cell_indices: pointer,
                               out_cells: ptr pointer,
                               out_proofs: ptr pointer): CResult {.importc: "recover_cells_and_proofs".}

proc constant_bytes_per_cell*(): uint64 {.importc: "constant_bytes_per_cell".}

proc constant_bytes_per_proof*(): uint64 {.importc: "constant_bytes_per_proof".}

proc constant_cells_per_ext_blob*(): uint64 {.importc: "constant_cells_per_ext_blob".}
