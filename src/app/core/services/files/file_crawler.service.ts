
import { Injectable, OnInit } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { AddToCrawlerQueueDTO } from "../../dtos/add-to-crawler-queue-dto";

@Injectable({ 'providedIn': 'root' })
export class FileCrawlerService {
    constructor() { }

    async addDirectoriesToQueue(directories: AddToCrawlerQueueDTO[]) {
        await invoke<void>("add_dirs_to_crawler_queue", { directories }).then(() => { }).catch(err =>
            console.log(err)
        )
        console.log(`Frontend validation: added ${directories.length} to the crawler queue`);
    }
}