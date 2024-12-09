import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { ConfigKeys, getDefaultConfig } from "./config-keys";
import { TauriCommandsService } from "../tauri/commands.service";

@Injectable({ 'providedIn': 'root' })
export class PersistentConfigService {
    private configFileName = "appconfig";

    private configSubject = new BehaviorSubject<ConfigKeys>(getDefaultConfig());
    public config$ = this.configSubject.asObservable();

    constructor(private commandsService: TauriCommandsService) { }

    /** Save the config  to disk */
    async save(): Promise<boolean> {
        const result = await this.commandsService.saveJsonLocal(this.configSubject.getValue(), this.configFileName);
        return result;
    }

    /** Load the config from disk */
    async load(): Promise<boolean> {
        console.log("Attempting to load JSON config from disk");
        const config = await this.commandsService.loadJsonLocal<ConfigKeys>(this.configFileName).catch(
            err => {
                console.log(`error loading config: ${err}`);
                return undefined;
            }
        );
        if (config) {
            this.configSubject.next(config);
            return true;
        }
        return false;
    }

    update<K extends keyof ConfigKeys>(key: K, data: ConfigKeys[K]) {
        const obj = { ...this.configSubject.getValue() };
        obj[key] = data;
        this.configSubject.next(obj);
    }

    read<K extends keyof ConfigKeys>(key: K): ConfigKeys[K] {
        const config = this.configSubject.getValue();
        return config[key];
    }

}