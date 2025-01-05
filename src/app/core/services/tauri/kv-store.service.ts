import { Injectable } from "@angular/core";
import { TauriCommandsService } from "./commands.service";
import { listen } from "@tauri-apps/api/event";
import { BehaviorSubject, Subject } from "rxjs";

@Injectable({ providedIn: "root" })
export class KvStorageService {
  constructor(private commandsService: TauriCommandsService) {}

  async set(key: string, value: any) {
    await this.commandsService.kvStoreSet(key, value);
  }

  async get<T>(key: string): Promise<T | undefined> {
    return this.commandsService.kvStoreGet(key);
  }

  async subscribe<T>(key: string): Promise<Subject<T>> {
    const model = await this.commandsService.kvStoreSubscribeToKey<T>(key);
    const ident: string = model.Identifier;
    const lastData: T | undefined = model.LastData;

    const subject = new Subject<T>();

    if (lastData) {
      subject.next(lastData);
    }

    // Listen to Tauri events and emit updates to the Subject
    const unlisten = await listen<T>(ident, (event) => {
      subject.next(event.payload);
    });

    // Add cleanup logic to unsubscribe when the Subject is complete
    const originalUnsubscribe = subject.unsubscribe.bind(subject);
    subject.unsubscribe = () => {
      // Clean up Tauri listener
      try {
        unlisten();
      } catch (err) {
        console.warn(`Failed to unlisten to KV subscription: ${err}`);
      }
      // Optional log
      console.log("KV subscription successfully unlistened from Tauri event");
      // Call the original unsubscribe logic
      originalUnsubscribe();
    };

    return subject;
  }
}
