
import { Injectable, OnInit } from "@angular/core";
import { AddToCrawlerQueueDTO } from "../../dtos/output/add-to-crawler-queue-dto";
import { IndexedDirModel } from "../../models/indexed-dir-model";
import { TauriCommandsService } from "../tauri/commands.service";

@Injectable({ 'providedIn': 'root' })
export class FileCrawlerService {
    constructor(private commandsService: TauriCommandsService) { }

    async addDirectoriesToQueue(directories: AddToCrawlerQueueDTO[]) {
        await this.commandsService.addDirsToCrawlerQueue(directories);
    }

    /**
     * 
     * @param limit The top n items to pull from the queue, as the queue is normally quite large
     * @returns items in the queue
     */
    async viewCrawlerQueue(limit: number): Promise<IndexedDirModel[]> {
        return await this.commandsService.viewCrawlerQueue(limit);
    }

    async viewCrawlerPriorityCounts(): Promise<Array<{ priority: number; count: number }>> {
        return await this.commandsService.viewCrawlerPriorityCounts();
    }

    async getCrawlerAnalyzerData(): Promise<Array<{ label: string, data: string }>> {
        return await this.commandsService.getCrawlerAnalyzerData();
    }
}