/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const BYTES_PER_COMMITMENT: number
export const BYTES_PER_PROOF: number
export const BYTES_PER_FIELD_ELEMENT: number
export const BYTES_PER_BLOB: number
export const MAX_NUM_COLUMNS: number
export const BYTES_PER_CELL: number
export class CellsAndProofs {
  cells: Array<Uint8Array>
  proofs: Array<Uint8Array>
}
export type DASContextJs = DasContextJs
export class DasContextJs {
  constructor()
  blobToKzgCommitment(blob: Uint8Array): Uint8Array
  asyncBlobToKzgCommitment(blob: Uint8Array): Promise<Uint8Array>
  computeCellsAndKzgProofs(blob: Uint8Array): CellsAndProofs
  asyncComputeCellsAndKzgProofs(blob: Uint8Array): Promise<CellsAndProofs>
  computeCells(blob: Uint8Array): Array<Uint8Array>
  asyncComputeCells(blob: Uint8Array): Promise<Array<Uint8Array>>
  recoverCellsAndKzgProofs(cellIndices: Array<bigint>, cells: Array<Uint8Array>): CellsAndProofs
  asyncRecoverCellsAndKzgProofs(cellIndices: Array<bigint>, cells: Array<Uint8Array>): Promise<CellsAndProofs>
  verifyCellKzgProofBatch(commitments: Array<Uint8Array>, cellIndices: Array<bigint>, cells: Array<Uint8Array>, proofs: Array<Uint8Array>): boolean
  asyncVerifyCellKzgProofBatch(commitments: Array<Uint8Array>, cellIndices: Array<bigint>, cells: Array<Uint8Array>, proofs: Array<Uint8Array>): Promise<boolean>
}
