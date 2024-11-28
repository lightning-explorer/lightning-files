import { Injectable } from "@angular/core";
import { FileDTO } from "../../../dtos/input/file-dto";
import { InlineQueryDTO } from "../../../dtos/output/inline-query-dto";
import { invoke } from "@tauri-apps/api/core";
import { FileModel } from "../../../models/file-model";
import { filterAlphanumeric } from "../../../../shared/services/keyboard-press-filter";
import { BehaviorSubject, Observable } from "rxjs";

@Injectable({ 'providedIn': 'root' })
export class InlineSearchService {

    private searchQuery = "";
    private firstOccurenceOfQueryIndexSubject = new BehaviorSubject<number>(0);
    firstOccurenceOfQueryIndex$ = this.firstOccurenceOfQueryIndexSubject.asObservable();

    async handleKeydown(event: KeyboardEvent, files: FileModel[]) {
        if (!this.isInputFocused()) {
            const key = filterAlphanumeric(event);
            if (event.key == "Backspace") {
                if (this.searchQuery.length == 0) {
                    return;
                }
                this.searchQuery = this.searchQuery.slice(0, -1);
            } else {
                if (key == undefined) {
                    return;
                }
                this.searchQuery += event.key;
            }
            console.log(this.searchQuery);
            const queryDto: InlineQueryDTO = { Query: this.searchQuery };
            const dtos = await this.query(queryDto);
            let foundFirst = false;

            for (let i = 0; i < files.length; i++) {
                let file = files[i];
                if (dtos.some(x => x.Name === file.Dto.Name)) {

                    foundFirst = true;
                    this.firstOccurenceOfQueryIndexSubject.next(i);

                    file.HighlightedText = this.searchQuery;
                } else {
                    file.HighlightedText = "";
                }
            }
        }
    }

    private isInputFocused(): boolean {
        const focusedElement = document.activeElement;
        const result = focusedElement && (focusedElement.tagName === 'INPUT' || focusedElement.tagName === 'TEXTAREA');
        return result ? result : false;
    }

    private async query(query: InlineQueryDTO): Promise<FileDTO[]> {
        return invoke<FileDTO[]>("search_files_inline", {
            query
        }).then(result =>
            result
        )
    }



}