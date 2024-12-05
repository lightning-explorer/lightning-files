import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FolderLoaderComponent } from './folder-loader.component';

describe('FolderLoaderComponent', () => {
  let component: FolderLoaderComponent;
  let fixture: ComponentFixture<FolderLoaderComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FolderLoaderComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(FolderLoaderComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
