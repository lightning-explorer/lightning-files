import { Component, Input } from '@angular/core';
import { DriveModel } from '../../../core/models/drive-model';
import { CommonModule } from '@angular/common';
import { IconifyIconModule } from '../../../shared/components/IconifyIcons/icon.module';
import { DirectoryNavigatorService } from '../../../core/services/files/directory-navigator.service';

@Component({
  selector: 'app-drive-result',
  standalone: true,
  imports: [CommonModule, IconifyIconModule],
  templateUrl: './drive-result.component.html',
  styleUrl: './drive-result.component.scss'
})
export class DriveResultComponent {
  @Input() drive: DriveModel | undefined;

  constructor(private directoryNavService: DirectoryNavigatorService) { }

  onClick() {
    if (this.drive)
      this.directoryNavService.setCurrentDir(this.drive.Name);
  }
}
