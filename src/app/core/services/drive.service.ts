import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { DriveModel } from "../models/drive-model";
import { invoke } from "@tauri-apps/api/core";

@Injectable({ 'providedIn': 'root' })
export class DriveService {
    private drivesSubject = new BehaviorSubject<DriveModel[]>([]);
    public drives$ = this.drivesSubject.asObservable();

    async refreshDrives() {
        await invoke<DriveModel[]>("get_drives").then(drives => {
            this.drivesSubject.next(drives)
        }
        ).catch(err =>
            console.log(err)
        )
    }
}