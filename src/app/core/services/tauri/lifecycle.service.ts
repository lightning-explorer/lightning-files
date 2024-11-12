import { Injectable } from "@angular/core";
import { PersistentConfigService } from "../persistence/config.service";


@Injectable({ 'providedIn': 'root' })
export class TauriLifecycleService {
    constructor(private configService: PersistentConfigService) { }

    async onStartup() {
        console.log("OnStartup called");
        this.configService.load();
    }

    async onShutdown() {
        console.log("OnShutdown called");
        this.configService.save();
    }
}