import nim_peerdas_kzg/bindings

import results
export results


# Note: there are no length checks in the nim code before calling the rust library because the types are
# are sized at compile time.

# TODO: If the underlying c library changes and we recompile the static lib
# TODO: nim will not recompile the tests. see test_yaml does not change for example
const
  FIELD_ELEMENTS_PER_BLOB = 4096
  FIELD_ELEMENTS_PER_CELL = 64
  BYTES_PER_FIELD_ELEMENT = 32
  CELLS_PER_EXT_BLOB = 128
  BlobSize* = FIELD_ELEMENTS_PER_BLOB*BYTES_PER_FIELD_ELEMENT
  CellSize* = FIELD_ELEMENTS_PER_CELL*BYTES_PER_FIELD_ELEMENT

type
  Bytes48* = object
    bytes*: array[48, byte]

  Blob* = object
    bytes*: array[BlobSize, byte]

  Cell* = object
    bytes*: array[CellSize, byte]

  KZGCommitment* = Bytes48
  
  KZGProof* = Bytes48

  Cells* = array[CELLS_PER_EXT_BLOB, Cell]

  CellsAndProofs* = object
    cells*: Cells
    proofs*: array[CELLS_PER_EXT_BLOB, KZGProof]


template getPtr(x: untyped): auto =
  when (NimMajor, NimMinor) <= (1,6):
    unsafeAddr(x)
  else:
    addr(x)

# Function to safely get a pointer to the first element of a sequence or openArray
template safeGetPtr[T](arr: openArray[T]): pointer =
  if arr.len > 0:
    arr[0].getPtr
  else:
    # Return a null pointer if the array is empty
    nil

# Convert an openArray of untyped to a pointer to a pointer
# ie convert a 2d array to a double pointer
template toPtrPtr(cells: openArray[untyped]): ptr pointer =
  # Create a seq of pointers to pointers
  var ptrSeq: seq[ptr pointer]
  ptrSeq.setLen(cells.len)
  
  # For each item in the openArray, get its pointer and assign it to the seq
  for i in 0..<cells.len:
    ptrSeq[i] = cast[ptr pointer](cells[i].bytes.getPtr)
  
  # Return the pointer to the seq of pointers
  cast[ptr pointer](ptrSeq.safeGetPtr)

template verify_result(res: CResult, ret: untyped): untyped =
  if res.xstatus != CResultStatus.Ok:
    # TODO: get error message then free the pointer
    return err($res)
  ok(ret)


type
  KZGCtx* = ref object
    ctx_ptr: ptr PeerDASContext

# Define custom destructor
# Nim2 does not allow us to take in a var T 
# for the custom destructor so it must ensure that
# this is not called twice.
# https://forum.nim-lang.org/t/11229
proc `=destroy`(x: typeof KZGCtx()[]) =
  if x.ctx_ptr != nil:
    peerdas_context_free(x.ctx_ptr)

proc newKZGCtx*(): KZGCtx =
  var kzgCtx = KZGCtx()
  kzgCtx.ctx_ptr = peerdas_context_new()
  return kzgCtx


proc blobToKZGCommitment*(ctx: KZGCtx, blob : Blob): Result[KZGCommitment, string] {.gcsafe.} =
  var ret: KZGCommitment
  
  let res = blob_to_kzg_commitment(
    ctx.ctx_ptr, 
    
    uint64(len(blob.bytes)),
    blob.bytes.getPtr, 
    
    ret.bytes.getPtr
  )
  verify_result(res, ret)


proc computeCellsAndProofs*(ctx: KZGCtx, blob : Blob): Result[CellsAndProofs, string] {.gcsafe.} =
  var ret: CellsAndProofs

  let outCellsPtr = toPtrPtr(ret.cells) 
  let outProofsPtr = toPtrPtr(ret.proofs) 
  
  let res = compute_cells_and_kzg_proofs(
    ctx.ctx_ptr,

    uint64(len(blob.bytes)),
    blob.bytes.getPtr,
    
    outCellsPtr,
    outProofsPtr
  )
  verify_result(res, ret)

proc computeCells*(ctx: KZGCtx, blob : Blob): Result[Cells, string] {.gcsafe.} =  
  let res = ?computeCellsAndProofs(ctx, blob)
  ok(res.cells)

proc verifyCellKZGProof*(ctx: KZGCtx, commitment: Bytes48, cellId: uint64, cell: Cell, proof: Bytes48): Result[bool, string] =
  var valid: bool

  let res =  verify_cell_kzg_proof(
    ctx.ctx_ptr, 
    
    uint64(len(cell.bytes)),
    cell.bytes.getPtr,
    
    uint64(len(commitment.bytes)),
    commitment.bytes.getPtr,
    
    cellId,

    uint64(len(proof.bytes)),
    proof.bytes.getPtr, 
    
    valid.getPtr
  )
  verify_result(res, valid)

proc verifyCellKZGProofBatch*(ctx: KZGCtx, rowCommitments: openArray[Bytes48],
                   rowIndices: openArray[uint64],
                   columnIndices: openArray[uint64],
                   cells: openArray[Cell],
                   proofs: openArray[Bytes48]): Result[bool, string] {.gcsafe.} =
  var valid: bool

  let cellsPtr = toPtrPtr(cells) 
  let proofsPtr = toPtrPtr(proofs) 
  let commitmentsPtr = toPtrPtr(rowCommitments)

  let res = verify_cell_kzg_proof_batch(
    ctx.ctx_ptr, 
    
    uint64(len(rowCommitments)), 
    commitmentsPtr, 
    
    uint64(len(rowIndices)),
    rowIndices.safeGetPtr, 
    
    uint64(len(columnIndices)), 
    columnIndices.safeGetPtr,
    
    uint64(len(cells)),
    cellsPtr, 
    
    uint64(len(proofs)),
    proofsPtr,

    valid.getPtr
  )
  verify_result(res, valid)


proc recoverCellsAndProofs*(ctx: KZGCtx,
                   cellIds: openArray[uint64],
                   cells: openArray[Cell]): Result[CellsAndProofs, string] {.gcsafe.} =
  
  var ret: CellsAndProofs
  
  let outCellsPtr = toPtrPtr(ret.cells) 
  let outProofsPtr = toPtrPtr(ret.proofs) 
  let inputCellsPtr = toPtrPtr(cells)

  let res = recover_cells_and_proofs(
    ctx.ctx_ptr,

    uint64(len(cells)),
    inputCellsPtr,
    
    uint64(len(cellIds)),
    cellIds.safeGetPtr,
    
    outCellsPtr,
    outProofsPtr,
  )

  verify_result(res, ret)

proc recoverCells*(ctx: KZGCtx,
                   cellIds: openArray[uint64],
                   cells: openArray[Cell]): Result[Cells, string] {.gcsafe.} =
  let res = ?recoverCellsAndProofs(ctx, cellIds, cells)
  ok(res.cells)