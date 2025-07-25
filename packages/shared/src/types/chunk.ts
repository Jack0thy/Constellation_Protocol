

/**
 * Represents a chunk of a file with its data and checksum
 */
export interface IChunk {
    /** The raw binary data of this chunk */
    data: Blob;
    /** SHA256 checksum of the chunk data */
    checksum: string;
}

/**
 * Represents a file that has been split into chunks for processing
 */
export interface IChunkedFile {
    /** SHA256 checksum of the original complete file */
    rawFileChecksum: string;
    /** Array of chunks that make up this file */
    chunks: IChunk[];
    /** Reconstructs the original file from its chunks */
    rebuild(): Promise<File>;
}
