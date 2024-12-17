import { Injectable } from "@angular/core";
import { DirectoryNavigatorService } from "./directory-navigator.service";

/**
 * Provides functionality for the undo and redo buttons
 */
@Injectable({ 'providedIn': 'root' })
export class DirectoryHistoryService{
    // Keep track of the previous directories the user had opened
    constructor(private directoryNavService:DirectoryNavigatorService){}

    /** Navigate to the previously visited directory, if any */
    undo(){

    }

    /**  Navigate back to the visited directory, if any */
    redo(){

    }
}