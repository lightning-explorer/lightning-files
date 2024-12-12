import { Injectable} from "@angular/core";
import { listen } from "@tauri-apps/api/event";
import { defaultParams, GetFilesParamsModel } from "../files/directory-navigator/models/get-files-params";
import { FileModel } from "../../models/file-model";
import { InvokeArgs, InvokeOptions } from "@tauri-apps/api/core";
import { DriveModel } from "../../models/drive-model";
import { InlineQueryDTO } from "../../dtos/output/inline-query-dto";
import { SearchParamsDTO } from "../../dtos/output/search-params-dto";
import { StreamingSearchParamsDTO } from "../../dtos/output/streaming-search-params-dtos";
import { AddToCrawlerQueueDTO } from "../../dtos/output/add-to-crawler-queue-dto";
import { IndexedDirModel } from "../../models/indexed-dir-model";

import { SafeInvokeService } from "./safe-invoke.service";

@Injectable({ 'providedIn': 'root' })
export class TauriCommandsService {

    constructor(private safeinvokeService: SafeInvokeService) { }

    async invokeSafe<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<T> {
        return await this.safeinvokeService.invokeSafe<T>(cmd, args, options);
    }

    async getFilesAsModels(directory: string, onEventEmit: (file: FileModel) => void, params?: GetFilesParamsModel) {
        if (!params)
            params = defaultParams();
        const unlisten = await listen<FileModel>("sys_file_model", (event) =>
            onEventEmit(event.payload)
        )
        try {
            await this.invokeSafe("get_files_as_models", { directory, params });
        }
        catch (err) {
            throw new Error(`${err}`);
        }
        finally {
            unlisten();
        }
    }

    async formatPathIntoDir(path: string): Promise<string> {
        return await this.invokeSafe<string | undefined>("format_path_into_dir", { path: path }).then((newPath) => {
            return newPath == undefined ? path : newPath;
        });
    }

    async getDirectoryPath(filePath: string): Promise<string> {
        return this.invokeSafe<string>("get_directory_path", {
            filePath
        }).then(path =>
            path
        )
    }

    async getRootPath(filePath: string): Promise<string> {
        return this.invokeSafe<string>("get_root_path", {
            filePath
        }).then(path =>
            path
        )
    }

    async getParentDirectory(filePath: string): Promise<string> {
        return this.invokeSafe<string>("get_parent_directory", {
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
        return this.invokeSafe<void>("open_file", {
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
        const fileContent = await this.invokeSafe<string>("read_file", {
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
        const fileContent = await this.invokeSafe<string>("read_file_range", {
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
        const fileContent = await this.invokeSafe<Uint8Array>("read_file_range_bytes", {
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
        return this.invokeSafe<boolean>("is_path_a_file", {
            filePath
        }).then(result =>
            result
        )
    }

    async getDrives(): Promise<DriveModel[]> {
        return await this.invokeSafe<DriveModel[]>("get_drives").then(drives => {
            return drives;
        }
        ).catch(err => {
            throw new Error(`${err}`);
        })
    }

    async searchFilesInline(query: InlineQueryDTO): Promise<FileModel[]> {
        return this.invokeSafe<FileModel[]>("search_files_inline", {
            query
        }).then(result =>
            result
        )
    }

    async searchIndexQuery(params: SearchParamsDTO): Promise<FileModel[]> {
        return this.invokeSafe<FileModel[]>("search_index_query", {
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
            await this.invokeSafe<Promise<void>>("search_index_query_streaming", { params });
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
        await this.invokeSafe<void>("save_json_local", {
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
        return await this.invokeSafe<T>("load_json_local", {
            name
        }).catch(err => {
            throw err;
        })
    }

    async addDirsToCrawlerQueue(directories: AddToCrawlerQueueDTO[]) {
        await this.invokeSafe<void>("add_dirs_to_crawler_queue", { directories }).then(() => { }).catch(err =>
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
        return await this.invokeSafe<IndexedDirModel[]>("view_crawler_queue", { limit }).catch(err => {
            console.log(err);
            return [];
        });
    }

    async viewCrawlerPriorityCounts(): Promise<Array<{ priority: number; count: number }>> {
        const record = await this.invokeSafe<Record<number, number>>("view_crawler_priority_counts").catch(err => {
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
        const record = await this.invokeSafe<Record<string, string>>("get_crawler_analyzer_data").catch(err => {
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

    /**
     * 
     * @param dir_path 
     * @returns `false` if a file path was provided or the directory was unable to be opened
     */
    async isDirectoryAccessible(dirPath: string): Promise<boolean> {
        return await this.invokeSafe<boolean>("is_directory_accessible", { dirPath });
    }
}