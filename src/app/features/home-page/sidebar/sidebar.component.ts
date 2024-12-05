import { Component, OnDestroy } from '@angular/core';
import { ButtonWIconComponent } from "../../../shared/components/buttons/button-w-icon/button-w-icon.component";
import { DriveService } from '../../../core/services/files/drive.service';
import { DriveModel } from '../../../core/models/drive-model';
import { CommonModule } from '@angular/common';
import { DriveResultComponent } from "../drive-result/drive-result.component";
import { ToolbarComponent } from "./toolbar/toolbar.component";
import { DropdownButtonComponent } from "../../../shared/components/buttons/dropdown-button/dropdown-button.component";
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-sidebar',
  standalone: true,
  imports: [CommonModule, DriveResultComponent, ToolbarComponent, DropdownButtonComponent],
  templateUrl: './sidebar.component.html',
  styleUrl: './sidebar.component.scss'
})
export class SidebarComponent implements OnDestroy {
  subscription = new Subscription();
  drives: DriveModel[] = [];

  constructor(private driveService: DriveService) {
    driveService.refreshDrives();
    this.subscription.add(this.driveService.drives$.subscribe(x =>
      this.drives = x
    ));
  }

  ngOnDestroy(): void {
    this.subscription.unsubscribe();
  }

  drivesButtonClick() {

  }

}
