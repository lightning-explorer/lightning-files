import { Component, Input } from '@angular/core';
import { FileDTOReceived } from '../../../core/services/dtos/file-dto-received';
import { CommonModule } from '@angular/common';
import {MatIconModule} from '@angular/material/icon'

@Component({
  selector: 'app-file-result',
  standalone: true,
  imports: [CommonModule, MatIconModule],
  templateUrl: './file-result.component.html',
  styleUrl: './file-result.component.scss'
})
export class FileResultComponent {
  @Input() file: FileDTOReceived | undefined;
}
