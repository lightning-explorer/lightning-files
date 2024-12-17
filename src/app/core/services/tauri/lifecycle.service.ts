import { Injectable } from "@angular/core";
import { PersistentConfigService } from "../persistence/config.service";
import { listen } from "@tauri-apps/api/event";
import { TauriCommandsService } from "./commands.service";
import { BehaviorSubject } from "rxjs";
import { invoke } from "@tauri-apps/api/core";
import { DirectoryNavigatorService } from "../files/directory-navigator/directory-navigator.service";

@Injectable({ 'providedIn': 'root' })
export class TauriLifecycleService {

    private isAppInitializedSubject = new BehaviorSubject<boolean>(false);
    isAppInitialized$ = this.isAppInitializedSubject.asObservable();

    constructor(private configService: PersistentConfigService,
        private directoryNavService: DirectoryNavigatorService) { }

    async onStartup() {
        this.initializeApp();
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
        this.directoryNavService.setCurrentDir("C:\\");
    }

    /**
     Put app-specific logic in here
     */
    async uninitializeApp() {
        this.configService.save();
    }
}