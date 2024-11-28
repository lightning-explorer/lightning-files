import { Injectable, input } from "@angular/core";
import { InlineQueryDTO } from "../../../dtos/output/inline-query-dto";
import { invoke } from "@tauri-apps/api/core";
import { FileModel } from "../../../models/file-model";
import { filterAlphanumeric } from "../../../../shared/services/keyboard-press-filter";
import { BehaviorSubject, Observable } from "rxjs";

/**
 Calls the Rust backend to handle the query operation
 */
@Injectable({ 'providedIn': 'root' })
export class InlineSearchService {

    private searchQuerySubject = new BehaviorSubject<string>("");
    searchQuery$ = this.searchQuerySubject.asObservable();

    private numberOfFoundItemsSubject = new BehaviorSubject<number>(0);
    numberOfFoundItems$ = this.numberOfFoundItemsSubject.asObservable();

    private firstOccurenceOfQueryIndexSubject = new BehaviorSubject<number>(0);
    firstOccurenceOfQueryIndex$ = this.firstOccurenceOfQueryIndexSubject.asObservable();

    async handleKeydown(event: KeyboardEvent, files: FileModel[]) {
        if (!this.isInputFocused()) {
            const key = filterAlphanumeric(event);
            let input_was_backspace = false;
            if (event.key == "Backspace") {
                if (this.searchQuerySubject.getValue().length == 0) {
                    return;
                }
                this.searchQuerySubject.next(this.searchQuerySubject.getValue().slice(0, -1));
                input_was_backspace = true;
            } else {
                if (key == undefined) {
                    return;
                }
                let searchQueryValue = this.searchQuerySubject.getValue();
                this.searchQuerySubject.next(searchQueryValue += event.key);
            }

            const queryDto: InlineQueryDTO = { Query: this.searchQuerySubject.getValue() };
            const dtos = await this.query(queryDto);

            this.numberOfFoundItemsSubject.next(dtos.length);

            for (let i = 0; i < files.length; i++) {
                let file = files[i];
                if (dtos.some(x => x.Name === file.Name)) {
                    if (!input_was_backspace)
                        this.firstOccurenceOfQueryIndexSubject.next(i);

                    file.HighlightedText = this.searchQuerySubject.getValue();
                } else {
                    file.HighlightedText = "";
                }
            }
        }
    }

    clearQuery() {
        this.searchQuerySubject.next("");
    }

    private isInputFocused(): boolean {
        const focusedElement = document.activeElement;
        const result = focusedElement && (focusedElement.tagName === 'INPUT' || focusedElement.tagName === 'TEXTAREA');
        return result ? result : false;
    }

    private async query(query: InlineQueryDTO): Promise<FileModel[]> {
        return invoke<FileModel[]>("search_files_inline", {
            query
        }).then(result =>
            result
        )
    }
}