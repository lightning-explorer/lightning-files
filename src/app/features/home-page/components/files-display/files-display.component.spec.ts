import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FilesDisplayComponent } from './files-display.component';

describe('FilesDisplayComponent', () => {
  let component: FilesDisplayComponent;
  let fixture: ComponentFixture<FilesDisplayComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FilesDisplayComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(FilesDisplayComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
