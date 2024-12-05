import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ExtendBarComponent } from './extend-bar.component';

describe('ExtendBarComponent', () => {
  let component: ExtendBarComponent;
  let fixture: ComponentFixture<ExtendBarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ExtendBarComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ExtendBarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
