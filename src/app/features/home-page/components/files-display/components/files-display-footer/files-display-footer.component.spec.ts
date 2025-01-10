import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FilesDisplayFooterComponent } from './files-display-footer.component';

describe('FilesDisplayFooterComponent', () => {
  let component: FilesDisplayFooterComponent;
  let fixture: ComponentFixture<FilesDisplayFooterComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FilesDisplayFooterComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(FilesDisplayFooterComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
