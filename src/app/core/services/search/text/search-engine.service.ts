import { HttpClient } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { SearchParamsDTO } from "../../../dtos/output/search-params-dto";
import { FileDTO} from "../../../dtos/input/file-dto";
import { environment } from "../../../../../environments/environment";

// Expects the 'SearchForJunk' service to be active
@Injectable({ 'providedIn': 'root' })
export class SearchEngineService {
    constructor(private http: HttpClient) { }

    async query(searchParams: SearchParamsDTO): Promise<FileDTO[]> {
        return new Promise((resolve, reject) => {
            const url = environment.searchapi.url;
            this.http.post<FileDTO[]>(`${url}/query`, searchParams).subscribe({
                next: (response) => {
                    resolve(response);
                },
                error: (error) => {
                    console.error('Search query error', error);
                    reject(error);
                },
            });
        });
    }
}