import { Component, Input, OnInit } from '@angular/core';
import { FileDTOReceived } from '../../../core/services/dtos/file-dto-received';
import { CommonModule } from '@angular/common';
import { FileResultComponent } from "../file-result/file-result.component";
import { Observable } from 'rxjs';

@Component({
  selector: 'app-files-display',
  standalone: true,
  imports: [CommonModule, FileResultComponent,],
  templateUrl: './files-display.component.html',
  styleUrl: './files-display.component.scss'
})
export class FilesDisplayComponent {
  @Input() files: FileDTOReceived[] = [];
  @Input() onFileClicked: ((file:FileDTOReceived)=>void)|undefined;
}
