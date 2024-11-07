import { Injectable } from "@angular/core";
import { invoke } from "@tauri-apps/api/core";
import { BehaviorSubject } from "rxjs";
import { ConfigKeys, getDefaultConfig } from "./config-keys";

@Injectable({ 'providedIn': 'root' })
export class PersistentConfigService {
    private configFileName = "appconfig";

    private configSubject = new BehaviorSubject<ConfigKeys>(getDefaultConfig());
    public config$ = this.configSubject.asObservable();

    /** Save the config  to disk */
    async save(): Promise<boolean> {
        await invoke<void>("save_json_local", {
            data: this.configSubject.getValue(),
            name: this.configFileName
        }).catch(x => {
            console.log(`error saving to JSON: ${x}`);
            return false;
        })

        return true;
    }

    /** Load the config from disk */
    async load(): Promise<boolean> {
        const config = await invoke<ConfigKeys>("load_json_local", {
            name: this.configFileName
        }).catch(x => {
            console.log(`error loading from JSON: ${x}`);
            return undefined;
        })
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