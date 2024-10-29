import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DriveResultComponent } from './drive-result.component';

describe('DriveResultComponent', () => {
  let component: DriveResultComponent;
  let fixture: ComponentFixture<DriveResultComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DriveResultComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(DriveResultComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
