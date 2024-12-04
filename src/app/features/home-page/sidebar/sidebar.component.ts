import { Component } from '@angular/core';
import { ButtonWIconComponent } from "../../../shared/components/buttons/button-w-icon/button-w-icon.component";
import { DriveService } from '../../../core/services/files/drive.service';
import { DriveModel } from '../../../core/models/drive-model';
import { CommonModule } from '@angular/common';
import { DriveResultComponent } from "../drive-result/drive-result.component";
import { ToolbarComponent } from "./toolbar/toolbar.component";
import { DropdownButtonComponent } from "../../../shared/components/buttons/dropdown-button/dropdown-button.component";

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [ButtonWIconComponent, CommonModule, DriveResultComponent, ToolbarComponent, DropdownButtonComponent],
  templateUrl: './sidebar.component.html',
  styleUrl: './sidebar.component.scss'
})
export class SidebarComponent {
  drives: DriveModel[] = [];

  constructor(private driveService: DriveService) {
    driveService.refreshDrives();
    this.driveService.drives$.subscribe(x =>
      this.drives = x
    );
  }

  drivesButtonClick(){
    
  }

}
