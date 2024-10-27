import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TestHtmlComponent } from './test-html.component';

describe('TestHtmlComponent', () => {
  let component: TestHtmlComponent;
  let fixture: ComponentFixture<TestHtmlComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [TestHtmlComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(TestHtmlComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
