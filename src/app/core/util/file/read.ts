import { invoke } from "@tauri-apps/api/core";

/**
 * Keep in mind that this function will read the entire contents of the file into memory and return it
 * @param filePath 
 * @returns 
 */
export async function readFileContents(filePath: string): Promise<string | undefined> {
    let error = "";
    const fileContent = await invoke<string>("read_file", {
        filePath
    }).catch(err =>
        error = err
    );
    if (fileContent) {
        return fileContent;
    }
    console.log(`Unable to read contents of file: ${error}`);
    return undefined;
}

/**
 * 
 * @param filePath 
 * @param start The byte to start reading at
 * @param length How many bytes to read
 * @returns The UTF8 encoded content of the file range
 */
export async function readFileRange(filePath: string, start:number, length:number): Promise<string | undefined> {
    let error = "";
    const fileContent = await invoke<string>("read_file_range", {
        filePath, start, length
    }).catch(err =>
        error = err
    );
    if (fileContent) {
        return fileContent;
    }
    console.log(`Unable to read contents of file: ${error}`);
    return undefined;
}

/**
 * 
 * @param filePath 
 * @param start The byte to start reading at
 * @param length How many bytes to read
 * @returns The raw bytes of the file range
 */
export async function readFileRangeBytes(filePath: string, start:number, length:number): Promise<Uint8Array | undefined> {
    let error = "";
    const fileContent = await invoke<Uint8Array>("read_file_range_bytes", {
        filePath, start, length
    }).catch(err =>
        error = err
    );
    if (fileContent) {
        return fileContent;
    }
    console.log(`Unable to read contents of file: ${error}`);
    return undefined;
}