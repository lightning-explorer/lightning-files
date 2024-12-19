import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FailedToMoveItemsPopupComponent } from './generic-err-popup.component';

describe('FailedToMoveItemsPopupComponent', () => {
  let component: FailedToMoveItemsPopupComponent;
  let fixture: ComponentFixture<FailedToMoveItemsPopupComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FailedToMoveItemsPopupComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(FailedToMoveItemsPopupComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
