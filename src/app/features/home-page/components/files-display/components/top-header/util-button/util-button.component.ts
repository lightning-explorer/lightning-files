import {
  AfterViewInit,
  Component,
  ElementRef,
  Input,
  OnDestroy,
  OnInit,
} from "@angular/core";
import { CommonModule } from "@angular/common";
import { DropdownButtonModalComponent } from "@shared/components/buttons/dropdown-button-modal/dropdown-button-modal.component";
import { IconifyIconModule } from "@shared/components/icons/IconifyIcons/icon.module";
import { FilesListService } from "../../../services/files-list.service";
import { SelectService } from "../../../services/select.service";
import { Subscription } from "rxjs";
import { FileModel } from "@core/models/file-model";
import { FileState } from "../../../../file-result/file-state";
import { TooltipDirective } from "@shared/components/popups/tooltip/tooltip.directive";
import { capitalizeFirstLetter } from "@shared/util/string";

interface File {
  model: FileModel;
  state: FileState;
}

export type UtilButtonType =
  | "copy"
  | "paste"
  | "new"
  | "rename"
  | "trash"
  | "cut"
  | "more";
@Component({
  selector: "app-util-button",
  standalone: true,
  imports: [
    IconifyIconModule,
    CommonModule,
    DropdownButtonModalComponent,
    TooltipDirective,
  ],
  templateUrl: "./util-button.component.html",
  styleUrl: "./util-button.component.css",
})
export class UtilButtonComponent implements OnInit, OnDestroy {
  private subscription = new Subscription();
  _isUsable = false;
  _lastSelectedFile?: File;
  _dropdownFeatures = [];
  _isDropdownType = false;
  _icon: string | undefined = undefined;
  
  _numSelectedItems = 0;

  constructor(
    private filesList: FilesListService,
    private selectService: SelectService
  ) {}

  @Input() type: UtilButtonType = "copy";

  get utilityName(): string {
    return capitalizeFirstLetter(this.type);
  }

  ngOnInit(): void {
    if (this.type == "new") this._isDropdownType = true;
    this._icon = this.type;

    this.subscription.add(
      this.selectService.lastSelectedItem$.subscribe((f) => {
        this._lastSelectedFile = f;
      })
    );
    this.subscription.add(
      this.selectService.selectedIndices$.subscribe((f) => {
        const n = f.size;
        this._numSelectedItems = n;
        if(n > 0 && !(this.type == "rename" && n != 1)){
          this._isUsable = true;
          return; 
        }
        this._isUsable = false;
      })
    );
  }

  onClick() {
    if (this.type == "rename") {
      if (this._lastSelectedFile)
        this._lastSelectedFile.state.requestRename = true;
      return;
    }
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }
}
