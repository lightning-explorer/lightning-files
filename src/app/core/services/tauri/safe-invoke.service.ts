import { Injectable, OnDestroy } from "@angular/core";
import { Subscription } from "rxjs";
import { invoke, InvokeArgs, InvokeOptions } from "@tauri-apps/api/core";

@Injectable({ providedIn: 'root' })
export class SafeInvokeService implements OnDestroy {
    private subscription = new Subscription();

    private appInitialized = false;

    constructor() { }

    /**
       * Returns `true` if the backend is active and all state has been fully managed.
       */
    async isRunning(): Promise<boolean> {
        return await invoke<boolean>("is_running").then(running => running).catch(_ => false);
    }

    /**
     * Invoke a Tauri command after all state has been initialized.
     */
    async invokeSafe<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<T> {
        if (!this.appInitialized) {
            const intervalId = setInterval(async () => {
                console.log("Pinging backend to see if it is active");
                const result = await this.isRunning();
                if (result) {
                    this.appInitialized = true;
                    clearInterval(intervalId); // Stop the interval when the function returns true
                    console.log('Backend responded to ping. All state has been managed');
                }
            }, 500);
        }
        return invoke<T>(cmd, args, options);
    }

    ngOnDestroy(): void {
        this.subscription.unsubscribe();
    }
}