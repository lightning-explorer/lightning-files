import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FileResultComponent } from './file-result.component';

describe('FileResultComponent', () => {
  let component: FileResultComponent;
  let fixture: ComponentFixture<FileResultComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FileResultComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(FileResultComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
