/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class ethereum_cryptography_LibEthKZG */

#ifndef _Included_ethereum_cryptography_LibEthKZG
#define _Included_ethereum_cryptography_LibEthKZG
#ifdef __cplusplus
extern "C" {
#endif
#undef ethereum_cryptography_LibEthKZG_BYTES_PER_COMMITMENT
#define ethereum_cryptography_LibEthKZG_BYTES_PER_COMMITMENT 48L
#undef ethereum_cryptography_LibEthKZG_BYTES_PER_PROOF
#define ethereum_cryptography_LibEthKZG_BYTES_PER_PROOF 48L
#undef ethereum_cryptography_LibEthKZG_BYTES_PER_FIELD_ELEMENT
#define ethereum_cryptography_LibEthKZG_BYTES_PER_FIELD_ELEMENT 32L
#undef ethereum_cryptography_LibEthKZG_BYTES_PER_BLOB
#define ethereum_cryptography_LibEthKZG_BYTES_PER_BLOB 131072L
#undef ethereum_cryptography_LibEthKZG_MAX_NUM_COLUMNS
#define ethereum_cryptography_LibEthKZG_MAX_NUM_COLUMNS 128L
#undef ethereum_cryptography_LibEthKZG_BYTES_PER_CELL
#define ethereum_cryptography_LibEthKZG_BYTES_PER_CELL 2048L
/*
 * Class:     ethereum_cryptography_LibEthKZG
 * Method:    DASContextNew
 * Signature: ()J
 */
JNIEXPORT jlong JNICALL Java_ethereum_cryptography_LibEthKZG_DASContextNew
  (JNIEnv *, jclass);

/*
 * Class:     ethereum_cryptography_LibEthKZG
 * Method:    DASContextDestroy
 * Signature: (J)V
 */
JNIEXPORT void JNICALL Java_ethereum_cryptography_LibEthKZG_DASContextDestroy
  (JNIEnv *, jclass, jlong);

/*
 * Class:     ethereum_cryptography_LibEthKZG
 * Method:    computeCellsAndKZGProofs
 * Signature: (J[B)Lethereum/cryptography/CellsAndProofs;
 */
JNIEXPORT jobject JNICALL Java_ethereum_cryptography_LibEthKZG_computeCellsAndKZGProofs
  (JNIEnv *, jclass, jlong, jbyteArray);

/*
 * Class:     ethereum_cryptography_LibEthKZG
 * Method:    blobToKZGCommitment
 * Signature: (J[B)[B
 */
JNIEXPORT jbyteArray JNICALL Java_ethereum_cryptography_LibEthKZG_blobToKZGCommitment
  (JNIEnv *, jclass, jlong, jbyteArray);

/*
 * Class:     ethereum_cryptography_LibEthKZG
 * Method:    verifyCellKZGProofBatch
 * Signature: (J[[B[J[[B[[B)Z
 */
JNIEXPORT jboolean JNICALL Java_ethereum_cryptography_LibEthKZG_verifyCellKZGProofBatch
  (JNIEnv *, jclass, jlong, jobjectArray, jlongArray, jobjectArray, jobjectArray);

/*
 * Class:     ethereum_cryptography_LibEthKZG
 * Method:    recoverCellsAndProof
 * Signature: (J[J[[B)Lethereum/cryptography/CellsAndProofs;
 */
JNIEXPORT jobject JNICALL Java_ethereum_cryptography_LibEthKZG_recoverCellsAndProof
  (JNIEnv *, jclass, jlong, jlongArray, jobjectArray);

#ifdef __cplusplus
}
#endif
#endif
