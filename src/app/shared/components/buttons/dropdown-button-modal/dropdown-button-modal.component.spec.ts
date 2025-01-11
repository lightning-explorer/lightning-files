import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DropdownButtonModalComponent } from './dropdown-button-modal.component';

describe('DropdownButtonModalComponent', () => {
  let component: DropdownButtonModalComponent;
  let fixture: ComponentFixture<DropdownButtonModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DropdownButtonModalComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(DropdownButtonModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
