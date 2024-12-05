import { ComponentFixture, TestBed } from '@angular/core/testing';

import { WindowsWindowChromeComponent } from './windows-window-chrome.component';

describe('WindowsWindowChromeComponent', () => {
  let component: WindowsWindowChromeComponent;
  let fixture: ComponentFixture<WindowsWindowChromeComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [WindowsWindowChromeComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(WindowsWindowChromeComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
