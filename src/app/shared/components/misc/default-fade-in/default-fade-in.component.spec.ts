import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DefaultFadeInComponent } from './default-fade-in.component';

describe('DefaultFadeInComponent', () => {
  let component: DefaultFadeInComponent;
  let fixture: ComponentFixture<DefaultFadeInComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DefaultFadeInComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(DefaultFadeInComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
