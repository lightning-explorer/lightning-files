import { ComponentFixture, TestBed } from '@angular/core/testing';

import { WindowChromeComponent } from './window-chrome.component';

describe('WindowChromeComponent', () => {
  let component: WindowChromeComponent;
  let fixture: ComponentFixture<WindowChromeComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [WindowChromeComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(WindowChromeComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
