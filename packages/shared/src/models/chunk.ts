import { IChunk, IChunkedFile } from "../types/chunk";
import { calculateChecksum } from "../utils/checksum";

/**
 * Represents a single chunk of a file with its data and checksum.
 * Implements the IChunk interface.
 */
export class Chunk implements IChunk {
    data: Blob;
    checksum!: string;

    private constructor(data: Blob) {
        this.data = data;
    }

    /**
     * Creates a new Chunk instance and calculates its checksum
     * @param data The binary data for this chunk
     * @returns Promise resolving to the new Chunk instance
     */
    static async create(data: Blob): Promise<Chunk> {
        const instance = new Chunk(data);
        await instance.calculateChecksum();
        return instance;
    }

    /**
     * Calculates and stores the SHA256 checksum for this chunk's data
     * @returns Promise resolving to the calculated checksum string
     */
    private async calculateChecksum(): Promise<string> {
        this.checksum = await calculateChecksum(this.data);
        return this.checksum;
    }
}

/**
 * Represents a file that has been split into chunks for processing.
 * Implements the IChunkedFile interface.
 */
export class ChunkedFile implements IChunkedFile {
    filename: string;
    fileType: string;
    rawFileChecksum!: string;
    chunks: IChunk[] = [];

    private constructor(file: File) {
        this.filename = file.name;
        this.fileType = file.type;
    }

    /**
     * Creates a new ChunkedFile instance, calculates checksums and splits into chunks
     * @param file The original File to process
     * @returns Promise resolving to the new ChunkedFile instance
     */
    static async create(file: File): Promise<ChunkedFile> {
        const instance = new ChunkedFile(file);
        await instance.calculateChecksum(file);
        instance.chunks = await instance.chunk(file);
        return instance;
    }

    /**
     * Calculates and stores the SHA256 checksum for the complete original file
     * @param file The original File to calculate checksum for
     * @returns Promise resolving to the calculated checksum string
     */
    private async calculateChecksum(file: File): Promise<string> {
        this.rawFileChecksum = await calculateChecksum(file);
        return this.rawFileChecksum;
    }

    /**
     * Splits a file into chunks of 2MB size
     * @param file The File to split into chunks
     * @returns Promise resolving to array of IChunk objects
     */
    private async chunk(file: File): Promise<IChunk[]> {
        const chunks: IChunk[] = [];
        const chunkSize = 1.8 * 1024 * 1024; // 1.8MB
        for (let i = 0; i < file.size; i += chunkSize) {
            const chunk = file.slice(i, i + chunkSize);
            chunks.push(await Chunk.create(chunk));
            console.log(`Chunk ${i} of ${file.size} created`);
        }
        console.log(`Total chunks: ${chunks}`);
        return chunks;
    }

    /**
     * Reconstructs the original file from its chunks
     * @returns Promise resolving to the reconstructed File
     */
    async rebuild(): Promise<File> {
        const blobs = this.chunks.map(chunk => chunk.data);
        return new File(blobs, this.filename, { type: this.fileType });
    }
}
