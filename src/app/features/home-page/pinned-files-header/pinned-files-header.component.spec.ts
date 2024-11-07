import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PinnedFilesHeaderComponent } from './pinned-files-header.component';

describe('PinnedFilesHeaderComponent', () => {
  let component: PinnedFilesHeaderComponent;
  let fixture: ComponentFixture<PinnedFilesHeaderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PinnedFilesHeaderComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(PinnedFilesHeaderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
