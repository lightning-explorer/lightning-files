import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MoveItemsPopupComponent } from './move-items-popup.component';

describe('MoveItemsPopupComponent', () => {
  let component: MoveItemsPopupComponent;
  let fixture: ComponentFixture<MoveItemsPopupComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MoveItemsPopupComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MoveItemsPopupComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
