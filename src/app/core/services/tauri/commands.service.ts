import { Injectable } from "@angular/core";
import { PersistentConfigService } from "../persistence/config.service";
import { listen } from "@tauri-apps/api/event";
import { defaultParams, GetFilesParamsModel } from "../files/directory-navigator/models/get-files-params";
import { FileModel } from "../../models/file-model";
import { invoke } from "@tauri-apps/api/core";
import { DriveModel } from "../../models/drive-model";
import { InlineQueryDTO } from "../../dtos/output/inline-query-dto";
import { SearchParamsDTO } from "../../dtos/output/search-params-dto";
import { StreamingSearchParamsDTO } from "../../dtos/output/streaming-search-params-dtos";
import { AddToCrawlerQueueDTO } from "../../dtos/output/add-to-crawler-queue-dto";
import { IndexedDirModel } from "../../models/indexed-dir-model";

@Injectable({ 'providedIn': 'root' })
export class TauriCommandsService {

    async getFilesAsModels(directory: string, onEventEmit: (file: FileModel) => void, params?: GetFilesParamsModel) {
        if (!params)
            params = defaultParams();
        const unlisten = await listen<FileModel>("sys_file_model", (event) =>
            onEventEmit(event.payload)
        )
        try {
            await invoke("get_files_as_models", { directory, params });
        }
        catch (err) {
            throw new Error(`${err}`);
        }
        finally {
            unlisten();
        }
    }

    async formatPathIntoDir(path: string, prevPath: string): Promise<string> {
        return await invoke<string | undefined>("format_path_into_dir", { path: path }).then((newPath) => {
            return newPath == undefined ? prevPath : newPath;
        });
    }

    async getDirectoryPath(filePath: string): Promise<string> {
        return invoke<string>("get_directory_path", {
            filePath
        }).then(path =>
            path
        )
    }

    async getRootPath(filePath: string): Promise<string> {
        return invoke<string>("get_root_path", {
            filePath
        }).then(path =>
            path
        )
    }

    async getParentDirectory(filePath: string): Promise<string> {
        return invoke<string>("get_parent_directory", {
            filePath
        }).then(path =>
            path
        )
    }

    /**
     * Runs the command prompt command to open the given file in its default environment
     * @param filePath 
     * @returns `true` if the operation was successful
     */
    async openFile(filePath: string): Promise<boolean> {
        return invoke<void>("open_file", {
            filePath
        }).then(() =>
            true
        ).catch(() =>
            false
        )
    }

    /**
     * Keep in mind that this function will read the entire contents of the file into memory and return it
     * @param filePath 
     * @returns 
     */
    async readFile(filePath: string): Promise<string | undefined> {
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
    async readFileRange(filePath: string, start: number, length: number): Promise<string | undefined> {
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
    async readFileRangeBytes(filePath: string, start: number, length: number): Promise<Uint8Array | undefined> {
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

    async isPathAFile(filePath: string): Promise<boolean> {
        return invoke<boolean>("is_path_a_file", {
            filePath
        }).then(result =>
            result
        )
    }

    async getDrives(): Promise<DriveModel[]> {
        return await invoke<DriveModel[]>("get_drives").then(drives => {
            return drives;
        }
        ).catch(err => {
            throw new Error(`${err}`);
        })
    }

    async searchFilesInline(query: InlineQueryDTO): Promise<FileModel[]> {
        return invoke<FileModel[]>("search_files_inline", {
            query
        }).then(result =>
            result
        )
    }

    async searchIndexQuery(params: SearchParamsDTO): Promise<FileModel[]> {
        return invoke<FileModel[]>("search_index_query", {
            params
        }).then(result =>
            result
        ).catch((err) => {
            throw new Error(`${err}`);
        })
    }

    async searchIndexQueryStreaming(params: StreamingSearchParamsDTO, onEventEmit: (files: FileModel[]) => void) {
        const eventName = `${params.StreamIdentifier}:search_result`

        const unlisten = await listen<FileModel[]>(eventName, (event) =>
            onEventEmit(event.payload));

        try {
            await invoke<Promise<void>>("search_index_query_streaming", { params });
        }
        catch (err) {
            console.log("Error performing streamed query", err)
        }
        finally {
            unlisten();
        }
    }

    /** Saves the data locally to disk */
    async saveJsonLocal(data: object, name: string): Promise<boolean> {
        await invoke<void>("save_json_local", {
            data,
            name
        }).catch(x => {
            console.log(`error saving to JSON: ${x}`);
            return false;
        })

        return true;
    }

    /** Loads the locally saved JSON from disk */
    async loadJsonLocal<T extends object>(name: string): Promise<T> {
        return await invoke<T>("load_json_local", {
            name
        }).catch(err => {
            console.log(err)
        })
    }

    async addDirsToCrawlerQueue(directories: AddToCrawlerQueueDTO[]) {
        await invoke<void>("add_dirs_to_crawler_queue", { directories }).then(() => { }).catch(err =>
            console.log(err)
        )
        console.log(`Frontend validation: added ${directories.length} to the crawler queue`);
    }

    /**
     * 
     * @param limit The top n items to pull from the queue, as the queue is normally quite large
     * @returns items in the queue
     */
    async viewCrawlerQueue(limit: number): Promise<IndexedDirModel[]> {
        return await invoke<IndexedDirModel[]>("view_crawler_queue", { limit }).catch(err => {
            console.log(err);
            return [];
        });
    }

    async viewCrawlerPriorityCounts(): Promise<Array<{ priority: number; count: number }>> {
        const record = await invoke<Record<number, number>>("view_crawler_priority_counts").catch(err => {
            console.log(err);
            return undefined;
        });
        if (record) {
            return Object.entries(record).map(([priority, count]) => (
                { priority: Number(priority), count }
            ));
        }
        return [];
    }

    async getCrawlerAnalyzerData(): Promise<Array<{ label: string, data: string }>> {
        const record = await invoke<Record<string, string>>("get_crawler_analyzer_data").catch(err => {
            console.log(err);
            return undefined;
        });
        if (record) {
            return Object.entries(record).map(([label, data]) => (
                { label, data }
            ));
        }
        return [];
    }
}