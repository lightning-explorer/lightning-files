import { Injectable } from "@angular/core";
import { BehaviorSubject, distinctUntilChanged, map, Observable } from "rxjs";
import { ConfigKeys, getDefaultConfig } from "./config-keys";
import { TauriCommandsService } from "../tauri/commands.service";

@Injectable({ providedIn: "root" })
export class PersistentConfigService {
  private configLoaded = false;
  private configFileName = "appconfig";

  private configSubject = new BehaviorSubject<ConfigKeys>(getDefaultConfig());
  private config$ = this.configSubject.asObservable();

  private configLoadedPromise: Promise<void>;

  constructor(private commandsService: TauriCommandsService) {
    this.configLoadedPromise = this.loadConfig();
  }

  private async loadConfig(): Promise<void> {
    console.log("Attempting to load JSON config from disk");
    const config = await this.commandsService
      .loadJsonLocal<ConfigKeys>(this.configFileName)
      .catch((err) => {
        console.log(`Error loading config: ${err}`);
        return undefined;
      });

    this.configLoaded = true;
    if (config) {
      this.configSubject.next(config);
    }
  }

  /** Save the config to disk */
  async save(): Promise<boolean> {
    const result = await this.commandsService.saveJsonLocal(
      this.configSubject.getValue(),
      this.configFileName
    );
    return result;
  }

  /** Ensure the config is loaded before accessing */
  private ensureLoaded(): Promise<void> {
    if (this.configLoaded) {
      return Promise.resolve();
    }
    return this.configLoadedPromise;
  }

  /** Subscribe to changes for one certain field in the config */
  observeKey<K extends keyof ConfigKeys>(key: K): Observable<ConfigKeys[K]> {
    return this.config$.pipe(
      map((config) => config[key]),
      distinctUntilChanged()
    );
  }

  /** Update a specific key in the config */
  async update<K extends keyof ConfigKeys>(key: K, value: ConfigKeys[K]): Promise<void> {
    const currentConfig = this.configSubject.value;
    this.configSubject.next({
      ...currentConfig,
      [key]: value,
    });
    // TODO: debounce the save operation if needed
    await this.save();
  }
 
  /** Waits until the config is fully loaded, then returns the value */
  async read<K extends keyof ConfigKeys>(key: K): Promise<ConfigKeys[K]> {
    await this.ensureLoaded();
    const config = this.configSubject.getValue();
    return config[key];
  }

  /** Attempts to read an item from the config.
   *
   * Returns the fallback value if the config hasn't been loaded from disk yet.
   */
  readOrElse<K extends keyof ConfigKeys>(
    key: K,
    fallback: ConfigKeys[K]
  ): ConfigKeys[K] {
    if (!this.configLoaded) return fallback;
    const config = this.configSubject.getValue();
    return config[key];
  }
}
