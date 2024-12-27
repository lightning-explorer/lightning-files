import {
  Component,
  EventEmitter,
  Input,
  Optional,
  Output,
} from "@angular/core";
import { DriveModel } from "@core/models/drive-model";
import { CommonModule } from "@angular/common";
import { IconifyIconModule } from "@shared/components/icons/IconifyIcons/icon.module";
import { HomePageService } from "../../services/home-page.service";
import { DirectoryNavigatorService } from "../../services/directory-navigator.service";
import { ButtonWIconComponent } from "../../../../shared/components/buttons/button-w-icon/button-w-icon.component";

@Component({
  selector: "app-drive-result",
  standalone: true,
  imports: [CommonModule, IconifyIconModule, ButtonWIconComponent],
  templateUrl: "./drive-result.component.html",
  styleUrl: "./drive-result.component.scss",
})
export class DriveResultComponent {
  @Input() drive: DriveModel | undefined;

  constructor(
    private directoryNavService: DirectoryNavigatorService,
    @Optional() private homePageService: HomePageService
  ) {}
}
