import { Component, Input } from '@angular/core';
import { FileDTOReceived } from '../../../core/dtos/file-dto-received';
import { CommonModule } from '@angular/common';
import { MatIconModule } from '@angular/material/icon'
import { DirectoryNavigatorService } from '../../../core/services/directory-navigator.service';
import { IconifyIconModule } from '../../../shared/IconifyIcons/icon.module';

@Component({
  selector: 'app-file-result',
  standalone: true,
  imports: [CommonModule, MatIconModule, IconifyIconModule],
  templateUrl: './file-result.component.html',
  styleUrl: './file-result.component.scss'
})
export class FileResultComponent {
  @Input() file: FileDTOReceived | undefined;

  constructor(private directoryService: DirectoryNavigatorService) { }

  async onClick() {
    if (this.file) {
      const path = this.file.FilePath;
      if (await this.directoryService.isPathAFile(path)) {
        await this.directoryService.openFileCmd(path);
      } else {
        await this.directoryService.setCurrentDir(path);
      }
    }
  }
}
