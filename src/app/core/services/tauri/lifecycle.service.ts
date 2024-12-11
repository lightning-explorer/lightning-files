import { Injectable } from "@angular/core";
import { PersistentConfigService } from "../persistence/config.service";
import { listen } from "@tauri-apps/api/event";

@Injectable({ 'providedIn': 'root' })
export class TauriLifecycleService {
    constructor(private configService: PersistentConfigService) { }

    async onStartup() {
        //this.configService.load();
        if (localStorage.getItem("APP_INITIALIZED") == "true") {
            //this.initializeApp();
        }
        await listen<void>("READY", () => {
            console.log("READY has bee emit. Frontend is intializing.")
            this.initializeApp();
            localStorage.setItem("APP_INITIALIZED", "true");
        });
    }

    async onShutdown() {
        console.log("OnShutdown called");
        this.uninitializeApp();
    }

    /**
     Put app-specific logic in here
     */
    async initializeApp() {
        this.configService.load();
    }

    /**
     Put app-specific logic in here
     */
    async uninitializeApp() {
        this.configService.save();
    }
}