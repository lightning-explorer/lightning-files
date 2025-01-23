import { Injectable } from '@angular/core';

@Injectable({providedIn: 'root'})
export class FileNameResolverService {

    constructor() { }
    
    getFileNameFromFullPath(fullPath:string){
        return fullPath.split('\\').pop() ?? "";
    }
}