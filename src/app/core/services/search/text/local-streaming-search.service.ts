import { Injectable } from "@angular/core";
import { FileModel } from "../../../models/file-model";
import { BehaviorSubject } from "rxjs";
import { StreamingSearchParamsDTO } from "../../../dtos/streaming-search-params-dtos";
import { TauriCommandsService } from "../../tauri/commands.service";
import { SearchParamsDTO } from "@core/dtos/search-params-dto";

@Injectable({ providedIn: "root" })
export class LocalStreamingSearchService {
  private filesSubject = new BehaviorSubject<FileModel[]>([]);
  public files$ = this.filesSubject.asObservable();

  private lastSearchParamsSubject = new BehaviorSubject<StreamingSearchParamsDTO|undefined>(undefined);
  lastSearchParams$ = this.lastSearchParamsSubject.asObservable();

  constructor(private commandsService: TauriCommandsService) {}

  clearResults(){
    this.filesSubject.next([]);
  }

  async query(params: StreamingSearchParamsDTO) {
    this.filesSubject.next([]);
    this.lastSearchParamsSubject.next(params);
    const innerParams = params.Params;
    // Because fuzzy queries have a tendency to return junk results when a low character
    // count is given, ignore calling the query altogether if the word length doesn't suffice
    if (
      innerParams.QueryType === "Fuzzy" &&
      !this.fuzzyQueryIsAdequate(innerParams)
    )
      return; // Early return
    await this.commandsService.searchIndexQueryStreaming(
      params,
      (emittedFiles) => {
        // Check if the emitted result corresponds to the correct query.
        // The query string is stored in the metadata field
        if (emittedFiles.Metadata == params.Params.FilePath) {
          const currentFiles = this.filesSubject.getValue();
          this.filesSubject.next([...currentFiles, ...emittedFiles.Data]);
        }
      }
    );
  }

  fuzzyQueryIsAdequate(params: SearchParamsDTO): boolean {
    const minLen = 4;
    if (params.FilePath !== undefined && params.FilePath.length < minLen) {
      return false;
    }
    if (params.Name !== undefined && params.Name.length < minLen) {
      return false;
    }
    return true;
  }
}
