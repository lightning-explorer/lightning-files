import { Injectable } from "@angular/core";
import { BehaviorSubject, distinctUntilChanged, map, Observable } from "rxjs";
import { ConfigKeys, getDefaultConfig } from "./config-keys";
import { TauriCommandsService } from "../tauri/commands.service";

@Injectable({ providedIn: "root" })
export class PersistentConfigService {
  // TODO: ensure config is loaded before reading
  private configLoaded = false;
  private configFileName = "appconfig";

  private configSubject = new BehaviorSubject<ConfigKeys>(getDefaultConfig());
  private config$ = this.configSubject.asObservable();

  constructor(private commandsService: TauriCommandsService) {}

  /** Save the config  to disk */
  async save(): Promise<boolean> {
    const result = await this.commandsService.saveJsonLocal(
      this.configSubject.getValue(),
      this.configFileName
    );
    return result;
  }

  /** Load the config from disk */
  async load(): Promise<boolean> {
    console.log("Attempting to load JSON config from disk");
    const config = await this.commandsService
      .loadJsonLocal<ConfigKeys>(this.configFileName)
      .catch((err) => {
        console.log(`error loading config: ${err}`);
        return undefined;
      });
    this.configLoaded = true;
    if (config) {
      this.configSubject.next(config);
      return true;
    }
    return false;
  }

  /** Subscribe to changes for one certain field in the config */
  observeKey<K extends keyof ConfigKeys>(key: K): Observable<ConfigKeys[K]> {
    return this.config$.pipe(
        map(config => config[key]),
        distinctUntilChanged()
    );
}

  /** Update the entire config object */
  updateConfig(newConfig: Partial<ConfigKeys>) {
    const currentConfig = this.configSubject.value;
    this.configSubject.next({ ...currentConfig, ...newConfig });
  }

  /**  Update a specific key in the config */
  update<K extends keyof ConfigKeys>(key: K, value: ConfigKeys[K]): void {
    const currentConfig = this.configSubject.value;
    this.configSubject.next({
        ...currentConfig,
        [key]: value
    });
}

  read<K extends keyof ConfigKeys>(key: K): ConfigKeys[K] {
    const config = this.configSubject.getValue();
    return config[key];
  }
}
