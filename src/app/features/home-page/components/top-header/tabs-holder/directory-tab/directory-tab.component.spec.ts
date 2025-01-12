import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DirectoryTabComponent } from './directory-tab.component';

describe('DirectoryTabComponent', () => {
  let component: DirectoryTabComponent;
  let fixture: ComponentFixture<DirectoryTabComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DirectoryTabComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(DirectoryTabComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
