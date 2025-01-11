import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ExtendBarVerticalComponent } from './extend-bar-vertical.component';

describe('ExtendBarVerticalComponent', () => {
  let component: ExtendBarVerticalComponent;
  let fixture: ComponentFixture<ExtendBarVerticalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ExtendBarVerticalComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ExtendBarVerticalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
