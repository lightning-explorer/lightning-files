import { Injectable } from "@angular/core";
import { PersistentConfigService } from "../persistence/config.service";
import { listen } from "@tauri-apps/api/event";

@Injectable({ 'providedIn': 'root' })
export class TauriLifecycleService {
    constructor(private configService: PersistentConfigService) { }

    async onStartup() {
        await listen<void>("tauri://init", () => {
            console.log("tauri://init has been emitting. Frontend is intializing.")
            this.configService.load();
        });
    }

    async onShutdown() {
        console.log("OnShutdown called");
        this.configService.save();
    }
}