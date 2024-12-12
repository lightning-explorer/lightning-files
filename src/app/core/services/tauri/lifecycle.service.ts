import { Injectable } from "@angular/core";
import { PersistentConfigService } from "../persistence/config.service";
import { listen } from "@tauri-apps/api/event";
import { TauriCommandsService } from "./commands.service";

@Injectable({ 'providedIn': 'root' })
export class TauriLifecycleService {

    constructor(private configService: PersistentConfigService, private commandsService: TauriCommandsService) { }

    async onStartup() {
        const intervalId = setInterval(async () => {
            console.log("Pinging backend to see if it is active");
            const result = await this.commandsService.isRunning();
            if (result) {
                clearInterval(intervalId); // Stop the interval when the function returns true
                this.initializeApp();
                console.log('Backend is initialized. Setting up frontnend');
            }
        }, 500);
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