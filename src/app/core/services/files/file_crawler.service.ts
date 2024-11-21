
import { Injectable, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { AddToCrawlerQueueDTO } from "../../dtos/add-to-crawler-queue-dto";
import { IndexedDirModel } from "../../models/indexed-dir-model";

@Injectable({ 'providedIn': 'root' })
export class FileCrawlerService {
    constructor() { }

    async addDirectoriesToQueue(directories: AddToCrawlerQueueDTO[]) {
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