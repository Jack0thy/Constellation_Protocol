import CryptoJS from "crypto-js";

/**
 * Calculates SHA256 checksum of a Blob
 * @param data The Blob to calculate checksum for
 * @returns Promise that resolves to hex string of SHA256 hash
 */
export function calculateChecksum(data: Blob): Promise<string> {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => {
            // Convert blob to ArrayBuffer then to WordArray for CryptoJS
            const arrayBuffer = reader.result as ArrayBuffer;
            const wordArray = CryptoJS.lib.WordArray.create(arrayBuffer);
            // Calculate SHA256 hash
            const hash = CryptoJS.SHA256(wordArray);
            resolve(hash.toString());
        };
        reader.onerror = () => {
            reject(new Error('Failed to read file'));
        };
        reader.readAsArrayBuffer(data);
    });
}
